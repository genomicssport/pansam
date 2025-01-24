use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "rust-samtools",
    version = "1.0",
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// generate ids for all the samtools id present in the samfile.
    Generateid {
        /// provide the path to the sam file
        pathsam: String,
    },
    /// generate ids for the specific range in the samtools file.
    Generateidrange {
        /// provide the path for the samtools file
        pathsam: String,
        /// provide the start of the range.
        start: usize,
        /// provide the end of the range.
        end: usize,
    },
    /// extracts the specific ids from the samfile.
    Extractor {
        /// provide the path to the samfile
        pathsam: String,
        /// provide the path to the list
        pathlist: String,
    },
    /// view the color coded reads for the specific region
    Viewer {
        /// provide the path to the samfile
        pathsam: String,
        /// provide the start coordinate of the samfile
        genomestart: usize,
        /// provide the end coordinate of the samfile
        genomeend: usize,
    },
    /// view the colour coded regions for the tags
    TagViewer {
        /// provide the path to the samfile
        pathsam: String,
        /// provide the start coordinate of the samfile
        genomestart: usize,
        /// provide the end coordinate of the samfile
        genomeend: usize,
    },
    /// upstream and downstream regions,prankaligner and visualization of the alignments
    SamUpDownAlignView {
        /// provide the path for the sam file
        pathsam: String,
        /// provide the path for the list of the ids
        pathlist: String,
        /// provide the path to the fasta file
        pathfasta: String,
        /// provide the upstream region size
        upstream: usize,
        /// provide the downstream region size
        downstream: usize,
        /// provide the path to the prank aligner
        prankaligner: String,
    },
    /// allows for the extraction of the upstream and downstream regions from the samfile
    SamUpDownAlign {
        /// provide the path for the sam file
        pathsam: String,
        /// provide the path for the list of the ids
        pathlist: String,
        /// provide the path to the fasta file
        pathfasta: String,
        /// provide the upstream region size
        upstream: usize,
        /// provide the downstream region size
        downstream: usize,
    },
    /// allows for the filtering of the samfile
    Filter {
        /// provide the path to the samfile
        pathsam: String,
        /// provide the start coordinate to be used as a filter
        pathstart: usize,
    },
    /// allows for the filtering of the samfile with specified range
    FilterRange {
       /// provide the path to the samfile
       pathsam: String,
       /// provide the start coordinate to be used as filter
       start: usize,
       /// provide the end coordinate to be used as filter
       end: usize,
    },
}
