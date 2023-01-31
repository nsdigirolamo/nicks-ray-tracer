# Nick's Ray Tracer

A ray tracer written in Rust following the [Ray Tracing in One Weekend](https://raytracing.github.io/) book series. This is a follow-up to my [previous attempt](https://github.com/nsdigirolamo/ray-tracing-in-one-weekend) where I did the same exact thing but it was written in C instead.

I'm hoping to complete all three books for this project, and afterwards I might even do some more stuff like making it multithreaded or something. Below are some example outputs so far.

<p align="center" float="left">
    <img src="https://i3.lensdump.com/i/RGirMi.png" width="45%"/>
    <img src="https://i1.lensdump.com/i/RGigdm.png" width="45%"/>
    <img src="https://i.lensdump.com/i/RGi0PZ.png" width="45%"/>
</p>

The below image is 1920x1080, 1000 samples per pixel, and max bounce depth of 500. It's the final scene for the first book in the series. There is no multi-threading or anything like that so it took 2 hours and 13 minutes to render on my AMD Ryzen 3600 @ 3.6 GHz.

<p align="center">
    <img src="https://i2.lensdump.com/i/RGij4K.png" width="90%"/> 
</p>

I am not going to implement motion blur because I don't think it looks very good and it doesn't interest me. 

I am also not going to implement bounding volume hierarchies (for the time being) because they use pointers. I'm not a good enough Rust developer to figure out how to get Rust to work nicely with pointers. I understand conceptually how the C++ code from the book works. That's enough for me to carry on without actually implementing it.
