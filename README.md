# oci-loader

Prevents oracle from reclaiming/deleteing your idle instances throught loading memory. 

This script will automatically hold up 20% of your instance's memory, satisifying the requirements by oracle.  
When other applications need those memory, it will automatically relase/reduce the memory held by the loader. 

### Usuage: 
1. install rust on your platform https://www.rust-lang.org
2. download/git clone this repo: `git clone https://github.com/Alvin-He/oci-loader`
3. open the location that you downloaded the repo in, run: `cargo build -r`
4. wait for the build to finish and an executable called oci_loader would be generated in `<REPO DIRECTORY>/target/release/`
5. just invoke that executable directly for the loader to run, to exit you have to CTRL+C or kill it in a task manager

##### Optional: You can configure your OS to start this program on start up so it's always running

NOTE: It will take about a minute or 2 for the loader to ramp up. So don't worry about it if your memory usuage don't instantly hit 20% 
