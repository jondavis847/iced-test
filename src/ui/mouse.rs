pub fn get_clicked_node(&mut self, cursor: Cursor, mouse_button: &MouseButton) {
    match mouse_button {
        MouseButton::Left => self.left_clicked_node = None,
        MouseButton::Right => self.right_clicked_node = None,
        MouseButton::Middle => self.middle_clicked_node = None,
    }

    self.nodes.iter_mut().for_each(|(key, node)| {
        if node.is_nodebar {
            // use canvas position
            if let Some(cursor_position) = cursor.position() {
                node.is_clicked(cursor_position, mouse_button);
                match mouse_button {
                    MouseButton::Left => {
                        if node.is_left_clicked {
                            self.left_clicked_node = Some(key.clone());
                        }
                    }
                    MouseButton::Right => {
                        if node.is_right_clicked {
                            self.right_clicked_node = Some(key.clone());
                        }
                    }
                    MouseButton::Middle => {
                        if node.is_middle_clicked {
                            self.middle_clicked_node = Some(key.clone());
                        }
                    }
                }
            }
        } else {
            // use graph position
            if let Some(cursor_position) = cursor.position_in(self.graph.bounds) {
                node.is_clicked(cursor_position, mouse_button);
                match mouse_button {
                    MouseButton::Left => {
                        if node.is_left_clicked {
                            self.left_clicked_node = Some(key.clone());
                        }
                    }
                    MouseButton::Right => {
                        if node.is_right_clicked {
                            self.right_clicked_node = Some(key.clone());
                        }
                    }
                    MouseButton::Middle => {
                        if node.is_middle_clicked {
                            self.middle_clicked_node = Some(key.clone());
                        }
                    }
                }
            }
        }
    });
}

pub fn cursor_moved(&mut self, cursor: Cursor) {
    if let Some(clicked_node_id) = self.left_clicked_node {
        // a node is clicked and being dragged
        if let Some(clicked_node) = self.nodes.get_mut(&clicked_node_id) {
            if clicked_node.is_nodebar {
                if let Some(cursor_position) = cursor.position() {
                    clicked_node.translate_to(cursor_position);
                    self.cache.clear();
                }
            } else {
                if let Some(cursor_position) = cursor.position_in(self.graph.bounds) {
                    clicked_node.translate_to(cursor_position);
                    self.cache.clear();
                    self.last_graph_cursor_position = cursor_position;
                }
            }
        }
    } else {
        // no node is clicked, graph is translating if the cursor is clicked on the graph
        if self.is_pressed {
            if let Some(graph_cursor_position) = cursor.position_in(self.graph.bounds) {
                let delta = graph_cursor_position - self.last_graph_cursor_position;
                self.nodes.iter_mut().for_each(|(_, node)| {
                    if !node.is_nodebar {
                        node.translate_by(delta);
                    }
                });
                self.last_graph_cursor_position = graph_cursor_position;
                self.cache.clear();
            }
        }
    }
    if let Some(cursor_position) = cursor.position_in(self.graph.bounds) {
        if let Some(clicked_node_id) = self.right_clicked_node {
            //edge is being drawn
            if let Some(edge_id) = self.current_edge {
                //keep moving current_edge
                if let Some(edge) = self.edges.get_mut(&edge_id) {
                    edge.to = EdgeConnection::Point(cursor_position);
                }
            } else {
                //create a new edge
                let new_edge = Edge::new(
                    EdgeConnection::Node(clicked_node_id),
                    EdgeConnection::Point(cursor_position),
                );
                let new_edge_id = Uuid::new_v4();
                self.edges.insert(new_edge_id, new_edge);
                self.current_edge = Some(new_edge_id);
                //add the edge to the from node so that if from node is deleted, edge is deleted
                if let Some(from_node) = self.nodes.get_mut(&clicked_node_id) {
                    from_node.edges.push(new_edge_id);
                }
            }
            self.cache.clear();
        }
    }
}
pub fn left_button_pressed(&mut self, cursor: Cursor) {
    self.is_pressed = true;

    if let Some(left_clicked_time_1) = self.left_clicked_time_1 {
        if left_clicked_time_1.elapsed() > Duration::from_millis(200) {
            // too long for double click, just a single click
            self.left_clicked_time_1 = Some(Instant::now());
        } else {
            //within double click, but need to wait for second release
            self.left_clicked_time_2 = Some(Instant::now());
        }
    } else {
        // self.left_clicked_time_1 was None
        self.left_clicked_time_1 = Some(Instant::now());
    }

    self.get_clicked_node(cursor, &MouseButton::Left);
    if let Some(selected_node) = self.left_clicked_node {
        self.selected_node = Some(selected_node);
    } else {
        self.selected_node = None;
    }
    if let Some(graph_cursor_position) = cursor.position_in(self.graph.bounds) {
        self.last_graph_cursor_position = graph_cursor_position;
    }
}

pub fn left_button_released(&mut self, cursor: Cursor) {
    self.is_pressed = false;

    // Determine the type of mouse button release event
    let release_event = match (self.left_clicked_time_1, self.left_clicked_time_2) {
        (Some(clicked_time_1), Some(clicked_time_2)) => {
            let clicked_elapsed_time_1 = clicked_time_1.elapsed();
            let clicked_elapsed_time_2 = clicked_time_2.elapsed();

            if clicked_elapsed_time_1 <= Duration::from_millis(500)
                && clicked_elapsed_time_2 <= Duration::from_millis(200)
            {
                MouseButtonReleaseEvents::DoubleClick
            } else {
                MouseButtonReleaseEvents::SingleClick
            }
        }
        (Some(clicked_time_1), None) => {
            if clicked_time_1.elapsed() <= Duration::from_millis(200) {
                MouseButtonReleaseEvents::SingleClick
            } else {
                MouseButtonReleaseEvents::Held
            }
        }
        _ => MouseButtonReleaseEvents::Held,
    };

    // Reset click times appropriately
    if matches!(
        release_event,
        MouseButtonReleaseEvents::SingleClick | MouseButtonReleaseEvents::DoubleClick
    ) {
        self.left_clicked_time_1 = None;
        self.left_clicked_time_2 = None;
    }

    // Handle the click event on the node
    if let Some(clicked_node_id) = self.left_clicked_node {
        // First clear any selected_nodes
        self.nodes.iter_mut().for_each(|(_, node)| {
            node.is_selected = false;
        });
        if let Some(clicked_node) = self.nodes.get_mut(&clicked_node_id) {
            match release_event {
                MouseButtonReleaseEvents::DoubleClick => {
                    // Handle double click if needed
                }
                MouseButtonReleaseEvents::SingleClick => {
                    if !clicked_node.is_nodebar {
                        clicked_node.is_selected = true;
                    }
                }
                MouseButtonReleaseEvents::Held => {
                    if clicked_node.is_nodebar {
                        if let Some(cursor_position) = cursor.position_in(self.graph.bounds) {
                            self.last_graph_cursor_position = cursor_position;
                        }
                        self.modal = Some(clicked_node_id);
                    }
                    clicked_node.drop();
                }
            }
        }
        self.left_clicked_node = None;
    } else {
        // Clear all selections if no node was clicked
        self.nodes
            .values_mut()
            .for_each(|node| node.is_selected = false);
        self.selected_node = None;
    }

    // Clear the cache
    self.cache.clear();
}

pub fn right_button_pressed(&mut self, cursor: Cursor) {
    self.get_clicked_node(cursor, &MouseButton::Right);
    if let Some(graph_cursor_position) = cursor.position_in(self.graph.bounds) {
        self.last_graph_cursor_position = graph_cursor_position;
    }
}

pub fn right_button_released(&mut self, cursor: Cursor) {
    // are we dragging an edge?
    if let Some(edge_id) = self.current_edge {
        // is there a node close enough to snap to?
        if let Some(snappable_node_id) = self.get_snappable_node(cursor, &self.nodes) {
            // if this edge_id still has a value in the hashmap
            if let Some(active_edge) = self.edges.get_mut(&edge_id) {
                // add the snappable nodes id to the edges "to" field
                active_edge.to = EdgeConnection::Node(snappable_node_id);
                // if the snappable node is still in the nodes hashmap
                // TODO: may need to force this or it will panic on delete of the node if it doesnt exist in the hashmap
                if let Some(snappable_node) = self.nodes.get_mut(&snappable_node_id) {
                    // add this edge to it's vector of edges
                    snappable_node.edges.push(edge_id);
                }
            }
        } else {
            // nothing to connect to, drop the edge
            self.edges.remove(&edge_id);
        }
        self.current_edge = None;
    }
    self.right_clicked_node = None;
    self.cache.clear();
}