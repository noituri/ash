use std::hash::Hash;

use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};

use ash_core::prelude::{Simple, SimpleReason, Source as SvSource};

pub fn error<T>(source: &SvSource, err: Simple<T>)
where
    T: ToString + Hash + Eq,
{
    let err = err.map(|c| c.to_string());
    let location = source.location();
    let location = location.as_str();

    let report = Report::build(ReportKind::Error, location, err.span().start);
    let report = match err.reason() {
        SimpleReason::Unexpected => report
            .with_message(format!(
                "{}{}",
                if err.found().is_some() {
                    "Unexpected token found"
                } else {
                    "Unexpected end of input"
                },
                if err.expected().len() != 0 {
                    let expected = err
                        .expected()
                        .map(|e| match e {
                            Some(e) => e.to_owned(),
                            None => "end of input".to_owned(),
                        })
                        .collect::<Vec<_>>()
                        .join(", ");

                    format!(". expected {}", expected)
                } else {
                    "".to_owned()
                }
            ))
            .with_label(Label::new((location, err.span())).with_message(format!(
                "Unexpected {}",
                err
                    .found()
                    .map(|v| format!("token {}", v.fg(Color::Red)))
                    .unwrap_or("end of input".fg(Color::Red).to_string())
            ))),
        SimpleReason::Unclosed { span, delimiter } => report
            .with_message(format!("Unclosed delimiter {}", delimiter.fg(Color::Cyan)))
            .with_label(
                Label::new((location, span.clone()))
                    .with_message(format!("Unclosed delimiter {}", delimiter.fg(Color::Cyan)))
                    .with_color(Color::Cyan),
            )
            .with_label(
                Label::new((location, err.span()))
                    .with_message(format!(
                        "Must be closed before this {}",
                        err.found()
                            .unwrap_or(&"end of input".to_string())
                            .fg(Color::Red)
                    ))
                    .with_color(Color::Red),
            ),
        SimpleReason::Custom(msg) => report.with_message(msg).with_label(
            Label::new((location, err.span()))
                .with_message(format!("{}", msg.fg(Color::Red)))
                .with_color(Color::Red),
        ),
    };

    report
        .finish()
        .eprint((location, Source::from(source.inner())))
        .unwrap();
}
