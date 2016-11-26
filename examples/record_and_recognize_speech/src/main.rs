extern crate pocketsphinx;
extern crate sphinxad;

enum Language{
    English,
    Russian,
}

impl Language{
    fn print(&self) -> &'static str {
        match *self{
            Language::English => "english",
            Language::Russian => "russian",
        }
    }
}

//select your language
const language:Language = Language::English;

fn main(){
    println!("{}",match language{
        Language::English => "sphinx understands only phrases from english/grammar.jsgf. Say `exit` to exit",
        Language::Russian => "sphinx понимает только фразы из russian/grammar.jsgf. Cкажите `выход` чтобы выйти",
    });

    let (hiddenMarkovModel,grammar,vocabulary)=match language{
        Language::English => ("english/cmusphinx-en-us-ptm-5.2", "english/grammar.jsgf", "english/vocabulary.dict" ),
        Language::Russian => ("russian/zero_ru.cd_cont_4000", "russian/grammar.jsgf", "russian/vocabulary.dict" ),
    };

    let psConfig = pocketsphinx::CmdLn::init(true, &[
        "-hmm", hiddenMarkovModel,
        "-jsgf", grammar,
        "-dict", vocabulary,
        //"-samprate", "16000",
        "-logfn", "log.txt" // 2>log.txt
    ]).unwrap();

    let decoder = pocketsphinx::PsDecoder::init(psConfig);

    //open Microphone and start recording
    let mut device=sphinxad::AudioDevice::default_with_sps(16000).unwrap();
    device.start_recording().unwrap();

    let mut isSpeechStarted=false;
    let mut buffer = [0i16; 2048];

    decoder.start_utt(None).unwrap();

    loop{
        let microphoneData=match device.read(&mut buffer[..]).unwrap(){
            Some(d) => d,
            None => continue,
        };

        decoder.process_raw(microphoneData, false, false).unwrap();

        if decoder.get_in_speech() {
            if !isSpeechStarted {//begining of utterance
                isSpeechStarted=true;
                println!("listening speech");
            }
        }else{
            if isSpeechStarted {//end of utterance
                isSpeechStarted=false;
                println!("end of speech");

                decoder.end_utt().unwrap();

                match decoder.get_hyp() {
                    None => println!("Not recognized"),
                    Some((hypothesis, _utt_id, _score)) => {
                        println!("Recognized: {}", hypothesis);

                        for seg in decoder.seg_iter() {
                            println!("{} : {}, {}",seg.word(), seg.prob().ascr, seg.prob().lscr);
                        }

                        if hypothesis==match language {
                            Language::English => "EXIT",
                            Language::Russian => "выход",
                        }{
                            break;
                        }
                    },
                }

                decoder.start_utt(None).unwrap();
            }
        }
    }
}
