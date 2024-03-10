use aiken_lang::ast::{TraceLevel, Tracing};
use aiken_project::watch::{self, watch_project, with_project};
use clap::builder::{MapValueParser, PossibleValuesParser, TypedValueParser};
use std::{path::PathBuf, process};

#[derive(clap::Args)]
/// Build an Aiken project
pub struct Args {
    /// Path to project
    directory: Option<PathBuf>,

    /// Deny warnings; warnings will be treated as errors
    #[clap(short = 'D', long)]
    deny: bool,

    /// When enabled, re-run the command on file changes instead of exiting
    #[clap(short, long)]
    watch: bool,

    /// Also dump textual uplc
    #[clap(short, long)]
    uplc: bool,

    /// Filter traces to be included in the generated program(s).
    ///   - user-defined: only consider traces that you've explicitly defined (either through the
    ///   'trace' keyword of via the trace-if-false ('?') operator.
    ///   - compiler-generated: only included internal traces generated by the Aiken compiler, for
    ///   example in usage of 'expect'.
    ///   - all: include both user-defined and compiler-generated traces.
    /// [optional] [default: all]
    #[clap(short, long, value_parser=filter_traces_parser(), default_missing_value="all", verbatim_doc_comment)]
    filter_traces: Option<fn(TraceLevel) -> Tracing>,

    /// Choose the verbosity level of traces:
    ///   - silent: disable traces altogether
    ///   - compact: only culprit line numbers are shown on failures
    ///   - verbose: enable full verbose traces as provided by the user or the compiler
    /// [optional]
    #[clap(short, long, value_parser=trace_level_parser(), default_value_t=TraceLevel::Silent, verbatim_doc_comment)]
    trace_level: TraceLevel,
}

pub fn exec(
    Args {
        directory,
        deny,
        watch,
        uplc,
        filter_traces,
        trace_level,
    }: Args,
) -> miette::Result<()> {
    let result = if watch {
        watch_project(directory.as_deref(), watch::default_filter, 500, |p| {
            p.build(
                uplc,
                match filter_traces {
                    Some(filter_traces) => filter_traces(trace_level),
                    None => Tracing::All(trace_level),
                },
            )
        })
    } else {
        with_project(directory.as_deref(), deny, |p| {
            p.build(
                uplc,
                match filter_traces {
                    Some(filter_traces) => filter_traces(trace_level),
                    None => Tracing::All(trace_level),
                },
            )
        })
    };

    result.map_err(|_| process::exit(1))
}

#[allow(clippy::type_complexity)]
pub fn filter_traces_parser(
) -> MapValueParser<PossibleValuesParser, fn(String) -> fn(TraceLevel) -> Tracing> {
    PossibleValuesParser::new(["user-defined", "compiler-generated", "all"]).map(
        |s: String| match s.as_str() {
            "user-defined" => Tracing::UserDefined,
            "compiler-generated" => Tracing::CompilerGenerated,
            "all" => Tracing::All,
            _ => unreachable!(),
        },
    )
}

#[allow(clippy::type_complexity)]
pub fn trace_level_parser() -> MapValueParser<PossibleValuesParser, fn(String) -> TraceLevel> {
    PossibleValuesParser::new(["silent", "compact", "verbose"]).map(|s| match s.as_str() {
        "silent" => TraceLevel::Silent,
        "compact" => TraceLevel::Compact,
        "verbose" => TraceLevel::Verbose,
        _ => unreachable!(),
    })
}
