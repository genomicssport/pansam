# pansam
 - all rust-samtools combined into a single application. Indiviual repositories removed and combined into a single application. Specific for long read and adding multi-threading to each part. 

 ```
 cargo build
 ```
 
 ```
 Usage: pansam <COMMAND>

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
