use std::fmt::Display;

#[macro_export]
macro_rules! docstr {
    ($line:expr) => {
        $crate::util::DocStr::line($line)
    };
}

#[macro_export]
macro_rules! docstr_empty {
    () => {
        $crate::util::DocStr::empty()
    };
    ($amount:expr) => {
        $crate::util::DocStr::empty_lines($amount)
    };
}

#[macro_export]
macro_rules! docstr_multi {
    ($lines:expr) => {
        $crate::util::DocStr::multiline($lines)
    };
    ($($line:expr),+) => {
        $crate::util::DocStr::multiline([$($line),+])
    };
}

#[macro_export]
macro_rules! docstr_multi_iter {
    ($lines:expr) => {
        $crate::util::DocStr::multiline_iter($lines)
    };
}

#[derive(Clone)]
pub enum DocStr {
    Line(String),
    MultiLine(Vec<String>),
}

impl DocStr {
    pub fn line(line: impl Into<String>) -> Self {
        let line: String = line.into();
        let maybe_lines = line.lines().collect::<Vec<_>>();

        match maybe_lines.len() {
            1 => Self::line(line),
            _ => Self::multiline(maybe_lines),
        }
    }

    pub fn multiline(lines: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self::multiline_iter(lines.into_iter())
    }

    pub fn multiline_iter(lines: impl Iterator<Item = impl Into<String>>) -> Self {
        Self::MultiLine(lines.map(Into::into).collect())
    }

    pub fn empty() -> Self {
        docstr!("")
    }

    pub fn empty_lines(amount: usize) -> Self {
        docstr_multi_iter!((0..amount).map(|_| ""))
    }

    pub fn map<RS>(self, f: impl Fn(String) -> RS) -> Self
    where
        RS: Into<String>,
    {
        match self {
            Self::Line(line) => docstr!(f(line)),
            Self::MultiLine(lines) => docstr_multi_iter!(lines.into_iter().map(f)),
        }
    }

    pub fn tabbed(self, tabs: usize) -> Self {
        match tabs {
            0 => self,
            _ => self.map(|s| format!("{}{s}", "\t".repeat(tabs))),
        }
    }

    pub fn into_lines(self) -> Vec<String> {
        match self {
            Self::Line(line) => vec![line],
            Self::MultiLine(lines) => lines,
        }
    }

    pub fn merge(self, other: impl Into<Self>) -> Self {
        let merged_lines = {
            let mut lines = self.into_lines();
            lines.append(&mut other.into().into_lines());

            lines
        };

        Self::multiline(merged_lines)
    }

    pub fn attach_right(self, right: impl Into<DocStr>) -> Self {
        match (self, right.into()) {
            (Self::Line(line), Self::Line(rline)) => docstr!(format!("{line} {rline}")),
            (Self::Line(line), Self::MultiLine(mut rlines)) => match rlines.len() {
                0 => docstr!(line),
                1 => docstr!(line).attach_right(rlines.remove(0)),
                _ => {
                    let line_len = line.len();
                    let fline = docstr!(line).attach_right(rlines.remove(0));

                    fline.merge(docstr_multi_iter!(rlines.into_iter().flat_map(|rline| {
                        (docstr!(" ".repeat(line_len)).attach_right(rline)).into_lines()
                    })))
                }
            },
            (Self::MultiLine(mut lines), Self::Line(rline)) => match lines.len() {
                0 => rline.into(),
                1 => docstr!(lines.remove(0)).attach_right(rline),
                _ => (docstr!(lines.remove(0)).attach_right(rline)).merge(docstr_multi!(lines)),
            },
            (Self::MultiLine(mut lines), Self::MultiLine(mut rlines)) => {
                let lines_len = lines.len();
                let rlines_len = rlines.len();

                match (lines_len, rlines_len) {
                    (0, 0) => docstr_empty!(),
                    (0, _) => docstr_multi!(rlines),
                    (_, 0) => docstr_multi!(lines),
                    _ => match lines_len == rlines_len {
                        true => lines
                            .into_iter()
                            .zip(rlines)
                            .map(|(line, rline)| docstr!(line).attach_right(rline))
                            .reduce(Self::merge)
                            .unwrap_or(docstr_empty!()),
                        false => match lines_len > rlines_len {
                            true => {
                                let attachable = lines.drain(0..rlines_len);

                                let attached = attachable
                                    .into_iter()
                                    .zip(rlines)
                                    .map(|(line, rline)| docstr!(line).attach_right(rline))
                                    .reduce(Self::merge)
                                    .unwrap();

                                lines.into_iter().fold(attached, |attached, other_line| {
                                    attached.merge(other_line)
                                })
                            }
                            false => {
                                let max_line_len =
                                    lines.iter().map(|line| line.len()).max().unwrap();

                                let rattachable = rlines.drain(0..rlines_len);

                                let rattached = rattachable
                                    .into_iter()
                                    .zip(lines)
                                    .map(|(rline, line)| docstr!(line).attach_right(rline))
                                    .reduce(Self::merge)
                                    .unwrap();

                                rlines.into_iter().fold(rattached, |rattached, rline| {
                                    rattached.merge(
                                        docstr!(" ".repeat(max_line_len)).attach_right(rline),
                                    )
                                })
                            }
                        },
                    },
                }
            }
        }
    }
}

impl<S> From<S> for DocStr
where
    S: Into<String>,
{
    fn from(value: S) -> Self {
        docstr!(value)
    }
}

impl Display for DocStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DocStr::Line(line) => line.fmt(f),
            DocStr::MultiLine(lines) => lines.join("\n").fmt(f),
        }
    }
}
