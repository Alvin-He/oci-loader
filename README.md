# oci-loader

Prevents oracle from reclaiming/deleteing your idle instances throught loading memory. 

This script will automatically hold up 20% of your instance's memory, satisifying the requirements by orcle.  
When other applications need those memory, it will automatically relase/reduce the memory held by the loader. 

Usuage: 
```
1. install rust on your platform https://www.rust-lang.org
2. download/git clone this repo: git clone https://github.com/Alvin-He/oci-loader
3. open the location that you downloaded the repo in, run: cargo build -r
4. wait for the build to finish and an executable called oci_loader would be generated in <REPO DIRECTORY>/target/release/
Optional: Configure your OS to start this program on start up so it's always running
```
