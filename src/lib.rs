use smallvec::SmallVec;

pub trait FastReplacePatterns {
    fn replace_patterns(&self, patterns: &[&str], values: &[String]) -> String;
}

pub trait StdReplacePatterns {
    fn replace_patterns(&self, patterns: &[&str], values: &[String]) -> String;
}

impl FastReplacePatterns for [u8] {
    fn replace_patterns(&self, patterns: &[&str], values: &[String]) -> String {
        let input_bytes = self;
        let mut pattern_pos = SmallVec::<[usize; 64]>::new();
        let mut stage = 0;
        for (i, &b) in input_bytes.iter().enumerate() {
            match (stage, b) {
                (1, b'{') => {
                    stage = 2;
                    pattern_pos.push(i);
                }
                (2, b'}') => {
                    stage = 0;
                    pattern_pos.push(i);
                }
                (_, b'%') => {
                    stage = 1;
                }
                _ => {}
            }
        }
        let mut output: Vec<u8> = Vec::with_capacity(input_bytes.len() + 128);
        let mut prev_end = 0;
        let pattern_values = patterns.iter().zip(values.iter());
        for pos in pattern_pos.chunks_exact(2) {
            let start = pos[0];
            let end = pos[1];
            let key = &input_bytes[start + 1..end];
            if prev_end < start {
                let prev_chunk = &input_bytes[prev_end..start - 1];
                output.extend_from_slice(prev_chunk);
            }
            if let Some((_, v)) = pattern_values
                .clone()
                .find(|(&pattern, _)| pattern.as_bytes() == key)
            {
                output.extend_from_slice(v.as_bytes());
            } else {
                output.extend_from_slice(&input_bytes[start - 1..end + 1]);
            }
            prev_end = end + 1;
        }
        if prev_end < input_bytes.len() {
            let remaining = &input_bytes[prev_end..];
            output.extend_from_slice(remaining);
        }
        unsafe { String::from_utf8_unchecked(output) }
    }
}

impl<T> FastReplacePatterns for T
where
    T: AsRef<[u8]>,
{
    fn replace_patterns(&self, patterns: &[&str], values: &[String]) -> String {
        self.as_ref().replace_patterns(patterns, values)
    }
}

impl<T> StdReplacePatterns for T
where
    T: AsRef<str>,
{
    fn replace_patterns(&self, patterns: &[&str], values: &[String]) -> String {
        let patterns: Vec<String> = patterns.iter().map(|s| format!("%{{{}}}", s)).collect();
        let mut output = self.as_ref().to_owned();
        for (i, pattern) in patterns.iter().enumerate() {
            output = output.replace(pattern, &values[i]);
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::FastReplacePatterns;

    #[test]
    fn replace_all_hello_world() {
        let patterns = &["greet", "what"];
        let values = &["Rust".to_owned(), "string".to_owned()];
        let input = "Hello %{greet}, this is a %{what} formating test!";
        let output = input.replace_patterns(patterns, values);
        assert_eq!(output, "Hello Rust, this is a string formating test!");
    }

    #[test]
    fn replace_all_begin_end() {
        let patterns = &["begin", "end"];
        let values = &["I'm".to_owned(), "the school!".to_owned()];
        let input = "%{begin} walking to %{end}";
        let output = input.replace_patterns(patterns, values);
        assert_eq!(output, "I'm walking to the school!");
    }

    #[test]
    fn replace_all_begin_end_reverse() {
        let patterns = &["end", "begin"];
        let values = &["the school!".to_owned(), "I'm".to_owned()];
        let input = "%{begin} walking to %{end}";
        let output = input.replace_patterns(patterns, values);
        assert_eq!(output, "I'm walking to the school!");
    }

    #[test]
    fn replace_all_no_var() {
        let patterns = &["begin", "end"];
        let values = &["I'm".to_owned(), "the school!".to_owned()];
        let input = "I'm walking to the school!";
        let output = input.replace_patterns(patterns, values);
        assert_eq!(output, "I'm walking to the school!");
    }

    #[test]
    fn replace_all_no_pattern() {
        let patterns = &[];
        let values = &[];
        let input = "%{begin} walking to the %{end}";
        let output = input.replace_patterns(patterns, values);
        assert_eq!(output, input);
    }
}
