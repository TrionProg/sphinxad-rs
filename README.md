# sphinxad-rs
Sphinxad hight-level wrapper, that allows to record voice from microphone and later recognize the speech by pocketsphinx

Usage
-----

Cargo.toml
```
sphinxad = "*"
```

You need two libraries to recognize speech:
* first should record voice from microphone(this [sphinxad-rs](https://github.com/TrionProg/sphinxad-rs))
* and second should process sound and recognize the speech([pocketsphinx-rs](https://github.com/kriomant/pocketsphinx-rs))

They are based on low-level libraries: [sphinxad-sys](https://github.com/TrionProg/sphinxad-sys) and [pocketsphinx-sys](https://github.com/kriomant/pocketsphinx-sys).
To discover how to use this libraries, look on [examples](https://github.com/TrionProg/sphinxad-rs/tree/master/examples).

You need HMM(Hidden Markov Model), dictionary and grammatics(or language model), to teach the pocketsphinx to understand phrases or run [examples](https://github.com/TrionProg/sphinxad-rs/blob/master/examples/record_and_recognize_speech).

This should be helpful:
[list of acoustic models for few languages](https://sourceforge.net/projects/cmusphinx/files/Acoustic%20and%20Language%20Models/)
* English. Download [HMM](https://sourceforge.net/projects/cmusphinx/files/Acoustic%20and%20Language%20Models/US%20English/cmusphinx-en-us-ptm-5.2.tar.gz/download) and [this dictionary](http://svn.code.sf.net/p/cmusphinx/code/trunk/cmudict/sphinxdict/cmudict_SPHINX_40), paste them into english directory as cmusphinx-en-us-ptm-5.2 and vocabulary.dict.
* Russian. Скачайте HMM [отсюда](https://sourceforge.net/projects/cmusphinx/files/Acoustic%20and%20Language%20Models/Russian/zero_ru_cont_8k_v3.tar.gz/download) и положите zero_ru.cd_cont_4000 в russian/. Для генерации словаря используйте [text2dict](https://github.com/zamiron/ru4sphinx/tree/master/text2dict) проекта [ru4sphinx](https://github.com/zamiron/ru4sphinx).

Dependencies
------------

In order to use the this crate, you must have the `libpocketsphinx`, `libsphinxad`, `libsphinxbase` libraries installed, you should use you packet manager or follow this [tutorial](http://cmusphinx.sourceforge.net/wiki/tutorialpocketsphinx)

Documentation
-------------

* [sphinxad-rs documentation](https://docs.rs/sphinxad/0.1.1/sphinxad/)
* [sphinxad official documentation](http://cmusphinx.sourceforge.net/doc/sphinxbase/ad_8h.html)

License
-------

MIT
