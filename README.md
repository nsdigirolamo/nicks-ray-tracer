# Nick's Ray Tracer

A ray tracer written in Rust following the [Ray Tracing in One Weekend](https://raytracing.github.io/) book series. This is a follow-up to my [previous attempt](https://github.com/nsdigirolamo/ray-tracing-in-one-weekend) where I did the
same exact thing but it was written in C instead.

I'm hoping to complete all three books for this project, and afterwards I might
even do some more stuff like making it multithreaded or something. Below are some
example outputs so far.

<p align="center">
    <img src="https://i.imgur.com/jj2K9NV.png"/>
    <img src="https://i.imgur.com/hnKXifY.png"/>
    <img src="https://i.imgur.com/AvZDz5O.png"/>
    <p>
        The below image is 1920x1080, 1000 samples per pixel, and max bounce depth 
        of 500. There is no multi-threading or anything like that so it took 2 hours
        and 13 minutes to render on my AMD Ryzen 3600 @ 3.6 GHz.
    </p>
    <img src="https://i.imgur.com/7AWGebu.png"/>
</center>
