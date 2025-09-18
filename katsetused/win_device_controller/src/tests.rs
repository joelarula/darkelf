#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fragment_buffer() {
        let mut buffer = FragmentBuffer::new(Duration::from_secs(5));
        
        // Test incomplete message
        assert!(!buffer.add_fragment("E0E1"));
        assert!(!buffer.is_complete());
        
        // Test complete message
        assert!(buffer.add_fragment("E2E3AABBEEFF"));
        assert!(buffer.add_fragment("E4E5E6E7"));
        assert!(buffer.is_complete());
        
        // Test message extraction
        let message = buffer.take_message();
        assert!(message.is_some());
        assert_eq!(message.unwrap(), "E0E1E2E3AABBEEFFFE4E5E6E7");
        assert!(buffer.buffer.is_empty());
    }

    #[tokio::test]
    async fn test_timeout() {
        let mut buffer = FragmentBuffer::new(Duration::from_millis(100));
        buffer.add_fragment("E0E1");
        
        // Wait for timeout
        tokio::time::sleep(Duration::from_millis(150)).await;
        assert!(buffer.is_timed_out());
    }
}