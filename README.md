# pansam

 - all rust-samtools combined into a single application specific for the long read technologies and async threaded.

 <img src="https://github.com/IBCHgenomic/pansam/blob/main/pansam.png" width="100" />

 ![](https://github.com/IBCHgenomic/eVaiutilities/blob/main/logo.png)

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

Gaurav Sablok \
Instytut Chemii Bioorganicznej Polskiej Akademii Nauk \
ul. Noskowskiego 12/14 | \
61-704, Poznań Poland
