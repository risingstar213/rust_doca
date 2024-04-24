use doca::comm_chan::CommChannel;

fn main() {
    let device = doca::device::open_device_with_pci("03:00.0").unwrap();
    let device_rep = doca::device::open_device_rep_with_pci(&device, "af:00.0").unwrap();

    let conn = CommChannel::create_server("cc_conn\0", &device, &device_rep);

    let send_txt = "hello dpu";
    let mut send_buffer = vec![0u8; 100].into_boxed_slice();
    let mut recv_buffer = vec![0u8; 100].into_boxed_slice();
    send_buffer.copy_from_slice(send_txt.as_bytes());

    let send_raw = RawPointer {
        inner: NonNull::new(send_buffer.as_mut_ptr() as *mut _).unwrap(),
        payload: send_txt.len(),
    };

    let mut recv_raw = RawPointer {
        inner: NonNull::new(recv_buffer.as_mut_ptr() as *mut _).unwrap(),
        payload: 0,
    };

    conn.block_recv_req(&mut recv_raw);

    conn.block_send_req(&send_raw);

    println!(
        "[After] recv_buffer check: {}",
        String::from_utf8(recv_buffer.to_vec()).unwrap()
    );

}