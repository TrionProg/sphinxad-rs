# sphinxad-rs
Sphinxad hight-level wrapper, that allows to record voice from microphone and later recognize speech by pocketsphinx

Usage
-----

You need two libraries to recognize speech:
* first should record voice from microphone(this [sphinxad-rs](https://github.com/TrionProg/sphinxad-rs))
* and second should process sound and recognize speech([pocketsphinx-rs](https://github.com/kriomant/pocketsphinx-rs))

They are based on low-level libraries: [sphinxad-sys](https://github.com/TrionProg/sphinxad-sys) and [pocketsphinx-sys](https://github.com/kriomant/pocketsphinx-sys).
To discover how to use this libraries, look on [examples](https://github.com/TrionProg/sphinxad-rs/tree/master/examples).

Dependencies
------------

In order to use the this crate, you must have the `libpocketsphinx`, `libsphinxad`, `libsphinxbase` libraries installed, you should use you packet manager or follow this [tutorial](http://cmusphinx.sourceforge.net/wiki/tutorialpocketsphinx)

Documentation
-------------

* [sphinxad-ad documentation](https://docs.rs/sphinxad-sys/0.1.0/sphinxad_ad/)
* [sphinxad official documentation](http://cmusphinx.sourceforge.net/doc/sphinxbase/ad_8h.html)

License
-------

MIT
