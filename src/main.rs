mod args;
mod capture;
mod extractor;
mod filter;
mod id;
mod idrange;
mod tagview;
mod view;
mod viewer;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::capture::capture;
use crate::extractor::extractor;
use crate::filter::filter;
use crate::id::fastid;
use crate::idrange::fastidrange;
use crate::tagview::tagview;
use crate::view::alignview;
use crate::viewer::readsview;
use clap::Parser;

/*
*Author Gaurav Sablok
*Universitat Potsdam and SLB Potsdam
*Date 2024-1-23
rust-samtools: provide all the abilities that you have to use with the
samtools and some more for the hybrid assay and also the development of
the approaches for the specific sequencing of certain region.

*/

///
fn main() {
    let samtoolsargs = CommandParse::parse();
    match &samtoolsargs.command {
        Commands::Generateid { pathsam } => {
            let commandout = fastid(pathsam).unwrap();
                println!("The fastid have been written:{:?}", commandout);
        }
        Commands::Generateidrange {
            pathsam,
            start,
            end,
        } => {
            let commandout = fastidrange(pathsam, *start, *end).unwrap();
            println!(
                "The idrange for the samtools have been written:{:?}",
                commandout
            );
        }
        Commands::Extractor { pathsam, pathlist } => {
            let commandout = extractor(pathsam, pathlist).unwrap();
            println!(
                "The extracted ids information are as follows: {:?}",
                commandout
            );
        }
        Commands::Viewer {
            pathsam,
            genomestart,
            genomeend,
        } => {
            let commandoutput = readsview(pathsam, *genomestart, *genomeend).unwrap();
            println!("The selected reads are:{:?}", commandoutput);
        }
        Commands::TagViewer {
            pathsam,
            genomestart,
            genomeend,
        } => {
            let commandout = tagview(pathsam, *genomestart, *genomeend).unwrap();
            println!("The selected reads are: {:?}", commandout);
        }
        Commands::SamUpDownAlignView {
            pathsam,
            pathlist,
            pathfasta,
            upstream,
            downstream,
            prankaligner,
        } => {
            let commandoutput = alignview(
                pathsam,
                pathlist,
                pathfasta,
                *upstream,
                *downstream,
                prankaligner,
            )
            .unwrap();
            println!(
                "The selected upstream and downstream regions have been
                aligned and the alignment is visible as embedded:{:?}",
                commandoutput
            );
        }
        Commands::SamUpDownAlign {
            pathsam,
            pathlist,
            pathfasta,
            upstream,
            downstream,
        } => {
            let commandoutput =
                capture(pathsam, pathlist, pathfasta, *upstream, *downstream).unwrap();
            println!(
                "The selected upstream and downstream regions have been
            aligned and the alignment is visible as embedded:{:?}",
                commandoutput
            );
        }
        Commands::Filter { pathsam, pathstart } => {
            let commandoutput = filter(pathsam, *pathstart).unwrap();
            println!("The requested samile has been filtered:{:?}", commandoutput);
        }
    }
}
