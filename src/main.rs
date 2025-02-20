use std::collections::HashMap;
use std::io::{self, BufRead};
use std::path::Path;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    let stdin = io::stdin();
    let handle = stdin.lock();

    // Read paths from stdin
    for line in handle.lines() {
        let path = line?.trim().to_string();
        if path.is_empty() {
            continue;
        }

        // Extract extension (in lowercase) or use empty string if none
        let ext = Path::new(&path)
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
            .unwrap_or_else(|| String::new());

        *counts.entry(ext).or_insert(0) += 1;
    }

    // Print counts sorted by count
    let mut counts: Vec<_> = counts.into_iter().collect();
    counts.sort_by_key(|(_, count)| *count);

    for (ext, count) in counts {
        let ext_display = if ext.is_empty() {
            "[no extension]".to_string()
        } else {
            format!(".{}", ext)
        };
        println!("{}: {}", ext_display, count);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn count_extensions(input: &[&str]) -> HashMap<String, usize> {
        let mut counts: HashMap<String, usize> = HashMap::new();
        for path in input {
            if path.is_empty() {
                continue;
            }
            let ext = Path::new(path)
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| e.to_lowercase())
                .unwrap_or_else(|| String::new());
            *counts.entry(ext).or_insert(0) += 1;
        }
        counts
    }

    #[tokio::test]
    async fn test_basic_extensions() {
        let input = vec![
            "file1.txt",
            "file2.txt",
            "image.png",
            "doc.pdf",
        ];
        let counts = count_extensions(&input).await;
        assert_eq!(counts.get("txt").unwrap(), &2);
        assert_eq!(counts.get("png").unwrap(), &1);
        assert_eq!(counts.get("pdf").unwrap(), &1);
    }

    #[tokio::test]
    async fn test_no_extensions() {
        let input = vec![
            "file1",
            "file2",
            "README",
        ];
        let counts = count_extensions(&input).await;
        assert_eq!(counts.get("").unwrap(), &3);
    }

    #[tokio::test]
    async fn test_mixed_case_extensions() {
        let input = vec![
            "file1.TXT",
            "file2.txt",
            "image.PNG",
            "doc.Pdf",
        ];
        let counts = count_extensions(&input).await;
        assert_eq!(counts.get("txt").unwrap(), &2);
        assert_eq!(counts.get("png").unwrap(), &1);
        assert_eq!(counts.get("pdf").unwrap(), &1);
    }

    #[tokio::test]
    async fn test_empty_input() {
        let input: Vec<&str> = vec![];
        let counts = count_extensions(&input).await;
        assert!(counts.is_empty());
    }

    #[tokio::test]
    async fn test_empty_lines() {
        let input = vec![
            "file1.txt",
            "",
            "file2.txt",
            "",
        ];
        let counts = count_extensions(&input).await;
        assert_eq!(counts.get("txt").unwrap(), &2);
    }
}
