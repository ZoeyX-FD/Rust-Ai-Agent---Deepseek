#[cfg(test)]
mod tests {
    use super::super::{ShortTermMemory, LongTermMemory};
    use std::fs;

    #[test]
    fn test_short_term_memory() {
        let mut stm = ShortTermMemory::new();
        
        // Test adding conversations
        stm.add_interaction(
            "What is machine learning?",
            "Machine learning is a branch of AI that enables systems to learn from data."
        );
        stm.add_interaction(
            "Tell me about neural networks",
            "Neural networks are computing systems inspired by biological neural networks."
        );

        // Test context retrieval
        let context = stm.get_context("");
        assert!(context.contains("machine learning"));
        assert!(context.contains("neural networks"));

        // Test conversation count
        assert_eq!(stm.conversation_count(), 2);
    }

    #[test]
    fn test_long_term_memory() {
        let mut ltm = LongTermMemory::new();
        
        // Test storing memories
        ltm.add_memory(
            "How do I implement a binary search?",
            "Here's how to implement binary search..."
        );

        // Test saving to file
        let test_file = "test_memory.json";
        ltm.save_to_file(test_file).expect("Failed to save memory");

        // Test loading from file
        let loaded_ltm = LongTermMemory::load_from_file(test_file).expect("Failed to load memory");
        
        // Verify loaded memory
        assert!(loaded_ltm.retrieve("How do I implement a binary search?").is_some());

        // Cleanup
        fs::remove_file(test_file).ok();
    }
} 