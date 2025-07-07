

pub struct StreamStatics {
    recv_bytes: usize,
    send_bytes: usize,
    recv_count: usize,
    send_count: usize,
    last_recv_time: i64,
    last_send_time: i64,
    last_recv_bytes: usize,
    last_send_bytes: usize,
    last_recv_count: usize,
    last_send_count: usize,
}

impl StreamStatics {
    pub fn new() -> Self {
        StreamStatics {
            recv_bytes: 0,
            send_bytes: 0,
            recv_count: 0,
            send_count: 0,
            last_recv_time: 0,
            last_send_time: 0,
            last_recv_bytes: 0,
            last_send_bytes: 0,
            last_recv_count: 0,
            last_send_count: 0,
        }
    }

    pub fn add_recv_bytes(&mut self, bytes: usize) {
        self.recv_bytes += bytes;
        self.recv_count += 1;
    }
    pub fn add_send_bytes(&mut self, bytes: usize) {
        self.send_bytes += bytes;
        self.send_count += 1;
    }
    pub fn get_recv_bytes(&self) -> usize {
        self.recv_bytes
    }
    pub fn get_send_bytes(&self) -> usize {
        self.send_bytes
    }
    pub fn get_recv_count(&self) -> usize {
        self.recv_count
    }
    pub fn get_send_count(&self) -> usize {
        self.send_count
    }

    pub fn get_recv_statics(&mut self) -> (f32, f32) {
        if self.last_recv_time == 0 {
            self.last_recv_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("时间回溯")
                .as_millis() as i64;
            self.last_recv_bytes = self.recv_bytes;
            self.last_recv_count = self.recv_count;
            return (0.0, 0.0);
        }

        let duration_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("时间回溯")
            .as_millis() as i64 - self.last_recv_time;
        let duration_bytes = self.recv_bytes - self.last_recv_bytes;
        let duration_count = self.recv_count - self.last_recv_count;

        if duration_ms <= 0 {
            return (0.0, 0.0);
        }
        self.last_recv_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("时间回溯")
            .as_millis() as i64;
        self.last_recv_bytes = self.recv_bytes;
        self.last_recv_count = self.recv_count;

        let recv_kbps = (duration_bytes as f32 * 8.0) / (duration_ms as f32);
        let recv_fps = (duration_count as f32) * 1000.0 / (duration_ms as f32);

        (recv_kbps, recv_fps)
    }

    pub fn get_send_statics(&mut self) -> (f32, f32) {
        if self.last_send_time == 0 {
            self.last_send_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("时间回溯")
                .as_millis() as i64;
            self.last_send_bytes = self.send_bytes;
            self.last_send_count = self.send_count;
            return (0.0, 0.0);
        }

        let duration_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("时间回溯")
            .as_millis() as i64 - self.last_send_time;
        let duration_bytes = self.send_bytes - self.last_send_bytes;
        let duration_count = self.send_count - self.last_send_count;

        if duration_ms <= 0 {
            return (0.0, 0.0);
        }
        self.last_send_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("时间回溯")
            .as_millis() as i64;
        self.last_send_bytes = self.send_bytes;
        self.last_send_count = self.send_count;

        let send_kbps = (duration_bytes as f32 * 8.0) / (duration_ms as f32);
        let send_fps = (duration_count as f32) * 1000.0 / (duration_ms as f32);

        (send_kbps, send_fps)
    }
    pub fn reset(&mut self) {
        self.recv_bytes = 0;
        self.send_bytes = 0;
        self.recv_count = 0;
        self.send_count = 0;
    }
}