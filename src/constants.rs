pub struct Constants {}
impl Constants {
    pub fn get_socket_path() -> String {
        "/tmp/multi-tasker/main/multi-tasker.sock".to_string()
    }
    pub fn get_status_file() -> String {
        "/tmp/multi-tasker/main/status.tmp".to_string()
    }
}
