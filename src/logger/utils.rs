pub fn process_start_trace(output: &str) {
  trace!("\n\nProcess Started: {}\n----------------", output);
}

pub fn process_end_trace() {
  trace!("\nProcess Finished\n----------------\n");
}
