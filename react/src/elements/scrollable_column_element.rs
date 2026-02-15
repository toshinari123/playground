use crate::prelude::{DisplayList, Element, Operation, Point, Size};
use std::sync::{Arc, Mutex};

// Manages scroll position and visibility calculations for a scrollable container.
#[derive(Debug)]
pub struct ScrollState {
    // Index of the first visible child
    offset: usize,
    // Total number of children (for wrapping)
    total_children: usize,
    // Number of currently visible children
    visible_count: usize,
}

impl ScrollState {
    pub fn new() -> Self {
        Self {
            offset: 0,
            total_children: 0,
            visible_count: 0,
        }
    }

    pub fn scroll_down(&mut self) {
        if self.total_children == 0 {
            return;
        }
        let max_offset = self.total_children.saturating_sub(self.visible_count.max(1));
        if self.offset < max_offset {
            self.offset += 1;
        } else {
            self.offset = 0;
        }
    }

    pub fn scroll_up(&mut self) {
        if self.total_children == 0 {
            return;
        }
        let max_offset = self.total_children.saturating_sub(self.visible_count.max(1));
        if self.offset > 0 {
            self.offset -= 1;
        } else {
            self.offset = max_offset;
        }
    }

    fn needs_scrollbar(&self) -> bool {
        self.total_children > self.visible_count
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
}

// Creates a scrollable column element with default scroll state.
pub fn scrollable_column(
    children: impl IntoIterator<Item = Box<dyn Element>>,
) -> ScrollableColumnElement {
    ScrollableColumnElement {
        children: children.into_iter().collect(),
        scroll_state: Arc::new(Mutex::new(ScrollState::new())),
    }
}

impl Element for ScrollableColumnElement {
    fn draw(&self, constraint: Size, display_list: &mut DisplayList) {
        if self.children.is_empty() || constraint.y <= 0 || constraint.x <= 0 {
            return;
        }

        draw_border(constraint, display_list);

        let inner_height = constraint.y - 2;
        let inner_width = constraint.x - 2;

        if inner_height <= 0 || inner_width <= 0 {
            return;
        }

        let content_width = inner_width - 1;
        let num_children = self.children.len();

        // Ask first child for preferred height, fallback to equal division
        let child_height = self.children[0]
        .preferred_size()
        .map(|s| s.y)
        .unwrap_or_else(|| (inner_height / num_children as isize).max(1));

        // All children get the same constraint
        let child_constraint = Size {
            x: content_width,
            y: child_height,
        };

        let start_index = {
            let state = self.scroll_state.lock().unwrap();
            state.offset.min(num_children.saturating_sub(1))
        };

        // Calculate how many children fit fully and if there's a partial
        let children_that_fit_fully = (inner_height / child_height) as usize;
        let remaining_space = inner_height % child_height;
        let has_partial = remaining_space > 0 
            && start_index + children_that_fit_fully < num_children;

        // Draw fully-fitting children
        let mut y_offset: isize = 0;
        let end_full = (start_index + children_that_fit_fully).min(num_children); // the index where we stop drawing full children. If start_index is 3 and children_that_fit_fully is 5, then end_full is 8.

        for i in start_index..end_full {
            let offset = Point { x: 1, y: 1 + y_offset };
            display_list.0.push(Operation::SetAnchor(offset));
            self.children[i].draw(child_constraint, display_list);
            display_list.0.push(Operation::SetAnchor(-offset));
            y_offset += child_height;
        }

        // Draw partial child if there's space and more children
        if has_partial {
            let partial_index = end_full;

            // Draw to temp display list (child_dl), then clip
            let mut child_dl = DisplayList::new();

            // let the child draw itself fully onto the temp display list
            self.children[partial_index].draw(child_constraint, &mut child_dl);

            // Copy from temp list to main list, but only keep what fits
            display_list.merge_clipped(
                &child_dl,      // source: the full child drawing
                1,              // x position: after left border
                1 + y_offset,   // y position: after top border + space taken by full children
                content_width,  // clip width: full width is fine
                remaining_space,// clip height: only 3 rows, not all 10 
            );
        }

        // Update scroll state
        let full_count = end_full - start_index;
        let visible_count = full_count + if has_partial { 1 } else { 0 };
        {
            let mut state = self.scroll_state.lock().unwrap();
            state.total_children = num_children;
            state.visible_count = visible_count.max(1);
        }

        let state = self.scroll_state.lock().unwrap();
        draw_scrollbar(
            constraint,
            inner_height,
            display_list,
            state.offset,
            num_children,
            full_count,
            state.needs_scrollbar(),
        );
    }
}

fn draw_border(size: Size, display_list: &mut DisplayList) {
    let ops = &mut display_list.0;
    let last_x = size.x - 1;
    let last_y = size.y - 1;

    // Top border
    ops.push(Operation::MoveTo(Point { x: 0, y: 0 }));
    ops.push(Operation::PutChar('┌'));
    for x in 1..last_x {
        ops.push(Operation::MoveTo(Point { x, y: 0 }));
        ops.push(Operation::PutChar('─'));
    }
    ops.push(Operation::MoveTo(Point { x: last_x, y: 0 }));
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
    for x in 1..last_x {
        ops.push(Operation::MoveTo(Point { x, y: last_y }));
        ops.push(Operation::PutChar('─'));
    }
    ops.push(Operation::MoveTo(Point { x: last_x, y: last_y }));
    ops.push(Operation::PutChar('┘'));
}

fn draw_scrollbar(
    size: Size,
    track_height: isize,
    display_list: &mut DisplayList,
    offset: usize,
    total: usize,
    visible: usize,
    needs_scrollbar: bool,
) {
    let ops = &mut display_list.0;

    // Scrollbar x position: right before the right border
    let x = size.x - 2;

    if track_height <= 0 {
        return;
    }

    if !needs_scrollbar || total == 0 || visible == 0 {
        // Draw empty track when no scrolling needed
        for y in 1..=track_height {
            ops.push(Operation::MoveTo(Point { x, y }));
            ops.push(Operation::PutChar('│'));
        }
        return;
    }

    // Calculate thumb dimensions
    let thumb_height = ((visible as f64 / total as f64) * track_height as f64)
        .round()
        .max(1.0) as isize;

    let scrollable_range = total.saturating_sub(visible);
    let thumb_start = if scrollable_range > 0 {
        ((offset as f64 / scrollable_range as f64) * (track_height - thumb_height) as f64)
            .round() as isize
    } else {
        0
    };

    // Draw track with thumb
    for y in 0..track_height {
        let screen_y = y + 1; // offset by 1 for top border
        ops.push(Operation::MoveTo(Point { x, y: screen_y }));

        let ch = if y >= thumb_start && y < thumb_start + thumb_height {
            '█'
        } else {
            '░'
        };
        ops.push(Operation::PutChar(ch));
    }

    // Draw arrow indicators
    if offset > 0 {
        ops.push(Operation::MoveTo(Point { x, y: 1 }));
        ops.push(Operation::PutChar('▲'));
    }

    if offset + visible < total {
        ops.push(Operation::MoveTo(Point { x, y: track_height }));
        ops.push(Operation::PutChar('▼'));
    }
}