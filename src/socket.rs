use tokio::{net::UdpSocket, sync::mpsc};

pub struct Server {
    pub socket: UdpSocket,
}

impl Server {
    pub async fn new(port: i16) -> Self {
        let socket = UdpSocket::bind(format!("0.0.0.0:{}", port)).await.unwrap();
        println!("UDP сервер запущен на порту {}", port);

        Self { socket }
    }

    pub async fn listen_packets(self, tx: mpsc::Sender<usize>) {
        let mut buf = [0; 1024];

        loop {
            // Ожидаем получение данных
            let (len, addr) = self.socket.recv_from(&mut buf).await.unwrap();

            // Преобразуем байты в строку (игнорируем ошибки преобразования)
            let received = String::from_utf8_lossy(&buf[..len]);
            let id = received.parse::<usize>();

            match id {
                Ok(data) => {
                    println!("Получено от {}: {}", addr, data);
                    tx.send(data).await.expect("Мы пизданулись.");
                }
                _ => {}
            }
        }
    }
}
