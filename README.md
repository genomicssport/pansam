# pangenome-samtools
 - all rust-samtools combined into a single application
 - The indiviual ones are located at these single repositories: 
 - [rust-samtools-genomeview](https://github.com/applicativesystem/rust-samtools-genomeview)
 - [rust-samtools-genomecapture](https://github.com/applicativesystem/rust-samtools-genomecapture)
 - [rust-samtools-generateidrange](https://github.com/applicativesystem/rust-samtools-generateid-range)
 - [rust-samtools-generateid](https://github.com/applicativesystem/rust-samtools-generateid)
 - [rust-samtools-filter](https://github.com/applicativesystem/rust-samtools-filter)
 - [rust-samtools-extractor](https://github.com/applicativesystem/rust-samtools-extractor)
 - [rust-samtools-multi-viewer](https://github.com/applicativesystem/rust-samtools-multi-viewer)

 ```
 cargo build
 ```
 
 ```
 Usage: rust-samtools <COMMAND>

 Commands:
  generateid              generate ids for all the samtools id present in the samfile
  generateidrange         generate ids for the specific range in the samtools file
  extractor               extracts the specific ids from the samfile
  viewer                  view the color coded reads for the specific region
  tag-viewer              view the colour coded regions for the tags
  sam-up-down-align-view  upstream and downstream regions,prankaligner and visualization of the alignments
  sam-up-down-align       allows for the extraction of the upstream and downstream regions from the samfile
  filter                  allows for the filtering of the samfile
  filter-range            allows for the filtering of the samfile with specified range
  help                    Print this message or the help of the given subcommand(s)

 Options:
  -h, --help     Print help
  -V, --version  Print version
 ``` 

 Gaurav Sablok
