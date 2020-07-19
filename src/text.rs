use std::iter;

#[derive(Debug, Clone)]
pub struct TextSpan {
    start: usize,
    end: usize,
}

impl TextSpan {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn single(position: usize) -> Self {
        Self {
            start: position,
            end: position + 1,
        }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn length(&self) -> usize {
        self.end - self.start
    }
}

#[derive(Debug, Clone)]
pub struct SourceCode {
    text: String,
    line_starts: Vec<usize>,
}

impl SourceCode {
    pub fn new(text: String) -> Self {
        let line_starts: Vec<usize> = iter::once(0)
            .chain(
                text.char_indices()
                    .filter_map(|(i, c)| if c == '\n' { Some(i + 1) } else { None }),
            )
            .collect();

        Self { text, line_starts }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn nth_line(&self, index: usize) -> &str {
        let start = self.line_starts[index];
        let end = self
            .line_starts
            .get(index + 1)
            .cloned();
        if let Some(end) = end {
            &self.text[start..end]
        } else {
            &self.text[start..]
        }
    }

    pub fn line_col(&self, index: usize) -> (usize, usize) {
        let line_starts = &self.line_starts;

        let mut low = 0;
        let mut high = line_starts.len() - 1;

        while low + 1 < high {
            let mid = low + (high - low) / 2;

            if index < self.line_starts[mid] {
                high = mid;
            } else {
                low = mid;
            }
        }
        let line = if index < line_starts[high] { low } else { high };

        (line + 1, index - line_starts[line] + 1)
    }
}
