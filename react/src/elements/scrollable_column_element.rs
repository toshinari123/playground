use crate::prelude::{Direction, DisplayList, Element, Operation, Point, Size};
use std::sync::{Arc, Mutex};

// Manages scroll position and visibility calculations for a scrollable container.
#[derive(Debug)]
pub struct ScrollState {
    // Index of the first visible child
    offset: usize,
    // Available height for children (excluding borders)
    viewport_height: isize,
    // Cached height of each child
    child_heights: Vec<isize>,
}

impl ScrollState {
    pub fn new() -> Self {
        Self {
            offset: 0,
            viewport_height: 0,
            child_heights: Vec::new(),
        }
    }

    pub fn scroll_down(&mut self) {
        let max_offset = self.max_scroll_offset();
        if self.offset < max_offset {
            self.offset += 1;
        } else {
            self.offset = 0;
        }
    }

    pub fn scroll_up(&mut self) {
        if self.offset > 0 {
            self.offset -= 1;
        } else {
            self.offset = self.max_scroll_offset();
        }
    }

    // Calculates the maximum valid scroll offset based on child count and viewport size.
    fn max_scroll_offset(&self) -> usize {
        let visible = self.visible_children_count();
        self.child_heights.len().saturating_sub(visible)
    }

    // Calculates how many children fit in the viewport starting from a given index.
    fn visible_children_count(&self) -> usize {
        let mut height_used = 0;
        let mut count = 0;

        // Iterate through children starting at the offset
        for height in self.child_heights.iter().skip(self.offset) {
            // Check if adding this child would overflow the viewport
            if height_used + height > self.viewport_height {
                break;
            }
            height_used += height;
            count += 1;
        }

        // Always show at least one item
        count.max(1) // max(x, 1)
    }

    fn needs_scrollbar(&self) -> bool {
        self.child_heights.len() > self.visible_children_count()
    }

    // Update scroll state based on current viewport size and child count. Called on each render.
    // # Arguments
    // * `viewport_height` - Available height for children (excluding borders)
    // * `new_children_count` - Total number of children in the container
    // * `min_child_height` - Minimum height each child should receive
    fn update(&mut self, viewport_height: isize, new_children_count: usize, min_child_height: isize) {
        self.viewport_height = viewport_height;
        let old_children_count = self.child_heights.len();

        // Recalculate child heights if count changed
        if old_children_count != new_children_count {
            // divide available height evenly among children
            let default_height = (viewport_height / new_children_count.max(1) as isize)
                // enforce minimum height 
                // TODO: where does it come from?
                .max(min_child_height);
            // create a list of default_heights of length new_children_count
            self.child_heights = vec![default_height; new_children_count];
        }

        // adjust offset in case children count decreased
        self.offset = self.offset.min(self.max_scroll_offset());
    }
}

impl Default for ScrollState {
    fn default() -> Self {
        Self::new()
    }
}

// A vertically scrollable container element.
pub struct ScrollableColumnElement {
    pub children: Vec<Box<dyn Element>>,
    pub scroll_state: Arc<Mutex<ScrollState>>,
    pub min_child_height: isize,
}

pub fn scrollable_column(
    children: impl IntoIterator<Item = Box<dyn Element>>,
) -> ScrollableColumnElement {
    ScrollableColumnElement {
        children: children.into_iter().collect(),
        scroll_state: Arc::new(Mutex::new(ScrollState::new())),
        min_child_height: 3,
    }
}

impl ScrollableColumnElement {
    const BORDER_WIDTH: isize = 1;
    const SCROLLBAR_WIDTH: isize = 1;

    pub fn min_child_height(mut self, height: isize) -> Self {
        self.min_child_height = height;
        self
    }
}

// renders the scrollable column element
impl Element for ScrollableColumnElement {
    fn draw(&self, constraint: Size, display_list: &mut DisplayList) {
        let inner = Size {
            x: constraint.x - 2 * Self::BORDER_WIDTH,
            y: constraint.y - 2 * Self::BORDER_WIDTH,
        };

        // Update scroll state with current dimensions
        {
            let mut state = self.scroll_state.lock().unwrap();
            state.update(inner.y, self.children.len(), self.min_child_height);
        }

        // Draw the border
        draw_column_border(constraint, display_list);

        let state = self.scroll_state.lock().unwrap();
        let needs_scrollbar = state.needs_scrollbar();
        let content_width = if needs_scrollbar {
            inner.x - Self::SCROLLBAR_WIDTH
        } else {
            inner.x
        };

        // Draw visible children
        let mut y_offset = Self::BORDER_WIDTH;
        let mut children_drawn = 0;

        for (i, child) in self.children.iter().enumerate().skip(state.offset) {
            let child_height = state.child_heights[i];

            if y_offset + child_height > constraint.y - Self::BORDER_WIDTH {
                break;
            }

            let offset = Point {
                x: Self::BORDER_WIDTH,
                y: y_offset,
            };

            display_list.0.push(Operation::SetAnchor(offset));
            child.draw(
                Size {
                    x: content_width,
                    y: child_height,
                },
                display_list,
            );
            display_list.0.push(Operation::SetAnchor(-offset));

            y_offset += child_height;
            children_drawn += 1;
        }

        // Draw scrollbar and indicators
        if needs_scrollbar {
            draw_scrollbar(
                constraint,
                display_list,
                state.offset,
                state.child_heights.len(),
                state.visible_children_count(),
            );
        }

        draw_scroll_indicators(
            constraint,
            display_list,
            state.offset,
            state.offset + children_drawn,
            state.child_heights.len(),
        );
    }
}

fn draw_column_border(size: Size, display_list: &mut DisplayList) {
    let ops = &mut display_list.0;
    let last_x = size.x - 1;
    let last_y = size.y - 1;

    // Top border
    ops.push(Operation::MoveTo(Point { x: 0, y: 0 }));
    ops.push(Operation::PutChar('┌'));
    for _ in 1..last_x {
        ops.push(Operation::Move(Direction::End));
        ops.push(Operation::PutChar('─'));
    }
    ops.push(Operation::Move(Direction::End));
    ops.push(Operation::PutChar('┐'));

    // Side borders
    for y in 1..last_y {
        ops.push(Operation::MoveTo(Point { x: 0, y }));
        ops.push(Operation::PutChar('│'));
        ops.push(Operation::MoveTo(Point { x: last_x, y }));
        ops.push(Operation::PutChar('│'));
    }

    // Bottom border
    ops.push(Operation::MoveTo(Point { x: 0, y: last_y }));
    ops.push(Operation::PutChar('└'));
    for _ in 1..last_x {
        ops.push(Operation::Move(Direction::End));
        ops.push(Operation::PutChar('─'));
    }
    ops.push(Operation::Move(Direction::End));
    ops.push(Operation::PutChar('┘'));
}

// The scrollbar consists of:
// - Track: A vertical line (┃) showing the full scrollable range
// - Thumb: A solid block (█) showing the current viewport position
fn draw_scrollbar(
    size: Size,
    display_list: &mut DisplayList,
    offset: usize,
    total: usize,
    visible: usize,
) {
    let ops = &mut display_list.0;

    // Scrollbar is positioned one column inside the right border
    let x = size.x - 2;

    // Track spans the inner height (excluding top and bottom borders)
    let track_height = size.y - 2;

    // Don't draw if there's no room or no need to scroll
    if track_height <= 0 || total <= visible {
        return;
    }

    // Calculate thumb dimensions
    let thumb_height = ((visible as f32 / total as f32) * track_height as f32)
        .round()
        .max(1.0) as isize;

    let scrollable_range = total.saturating_sub(visible);
    let thumb_y = if scrollable_range > 0 {
        ((offset as f32 / scrollable_range as f32) * (track_height - thumb_height) as f32)
            .round() as isize
    } else {
        0
    };

    // Draw track and thumb
    for y in 0..track_height {
        let screen_y = y + 1;
        ops.push(Operation::MoveTo(Point { x, y: screen_y }));

        let char = if y >= thumb_y && y < thumb_y + thumb_height {
            '█'
        } else {
            '┃'
        };
        ops.push(Operation::PutChar(char));
    }
}

fn draw_scroll_indicators(
    size: Size,
    display_list: &mut DisplayList,
    offset: usize,
    last_visible: usize,
    total: usize,
) {
    let ops = &mut display_list.0;
    let x = size.x - 2;

    if offset > 0 {
        ops.push(Operation::MoveTo(Point { x, y: 1 }));
        ops.push(Operation::PutChar('↑'));
    }

    if last_visible < total {
        ops.push(Operation::MoveTo(Point { x, y: size.y - 2 }));
        ops.push(Operation::PutChar('↓'));
    }
}