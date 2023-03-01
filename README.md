# ViccValentine's Status Updater (Rust)
Back in February 2022, I wrote a [Python script](https://github.com/LunarEcklipse/viccva_status_updater_python) to capture a friend's Twitch stream status using Twitch's RESTful API over the course of a couple of days. While it proved effective for what I needed, the script required a large amount of memory to operate for such a simple operation (something in the realm of 27 megabytes). This was pretty disappointing as there was a relatively tight memory constriction (being one of several processes operating on a Raspberry Pi), I decided to rewrite it in a properly compiled language. Since friends had been talking about Rust around the time I did this, I decided to try that.

## Things I learned from this:
* More about RESTful API utilization, especially how it works on a lower level
* Better techniques for the validation of incoming data
* Working within statically typed language requirements and .json data.
* How interpreted languages can differ greatly from those that are compiled in design, execution, and performance.
* Targeting a different platform than the one you're using during compilation (targeting ARM64 instead of x86_64).

## Things I would do differently:
* <b>Utilization of a mutex state.</b> Like with my Python script, I did not at the time understand how to control data access to my authorization variables while they were actively in use. I have since relearned those skills, which if applied to this code would make it thread-safe and usable within a program instead of as a separate one updating a document.
* <b>Possibly better memory optimization.</b> Since this project was ultimately about the optimization of processor time and memory, I ended up reducing the memory requirements by a a third (down to 9 megabytes of memory from 27), but I do not doubt that there are ways I could have reduced this further. Perhaps I could have done things like making my imports more specific or using more lightweight libraries to avoid importing things that could impact the performance more harshly. I haven't done much work in Rust since this script, but I would like to try writing more things in it when I have the time for it.
