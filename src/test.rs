#[cfg(test)]
mod tests {
    use super::*;
    use mockstream::SharedMockStream;
    use tempfile::NamedTempFile;

    #[test]
    fn test_handle_connection_found() {
        let temp_file = NamedTempFile::new().unwrap();
        let mut stream = SharedMockStream::new();
        write!(stream, "{}", GET).unwrap();

        let expected_response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}",
            status_line = FOUND,
            length = 0,
            contents = ""
        );

        let mut contents = Vec::new();
        handle_connection(stream.by_ref());
        stream.read_to_end(&mut contents).unwrap();

        assert_eq!(String::from_utf8(contents).unwrap(), expected_response);
    }

    #[test]
    fn test_handle_connection_not_found() {
        let temp_file = NamedTempFile::new().unwrap();
        let mut stream = SharedMockStream::new();
        write!(stream, "GET /not_found HTTP/1.1").unwrap();

        let expected_response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}",
            status_line = NOT_FOUND,
            length = 0,
            contents = ""
        );

        let mut contents = Vec::new();
        handle_connection(stream.by_ref());
        stream.read_to_end(&mut contents).unwrap();

        assert_eq!(String::from_utf8(contents).unwrap(), expected_response);
    }
}
