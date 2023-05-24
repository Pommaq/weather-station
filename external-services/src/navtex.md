# Notes on how to decode Navtex
https://www.allaboutcircuits.com/textbook/radio-frequency-analysis-design/radio-frequency-demodulation/how-to-demodulate-digital-phase-modulation/
https://thinkrf.com/solutions/signal-analysis-demodulation/
https://www.embedded.com/the-goertzel-algorithm/

we need to do a bit of math. We know Navtex transmit at 100 baud,
so every 1/100th second there is a "bit". We expect to read at
518Khz and our sample file is at 1953hz sample rate.
Thus assuming we start at 0 we expect to see a bit at
sample number x by calculating 1953/100 = 195.3*x

I will, for now, assume that if we see 518khz + 170 hz it's a 1.
if it's 518khz -170hz it's a 0.

It appears we can decode this FSK modulation using Goertzel algorithm.

Keyword for the "decoding" we want to do is "RF demodulation".

https://www.youtube.com/watch?v=h_7d-m1ehoY
By convention:
- The amplitude of the "in-phase" signal:
    i = I * cos (2*pi*f*t);
- The amplitude of the "90degree shifted" signal:
    q = Q * sin(2*pi*f*t);


Using that knowledge with
https://electronics.stackexchange.com/questions/132642/how-to-get-bpsk-data-from-i-q-signal
we now know we need the signal offset to calculate the amplitudes Q and I.
We know that NavTex use a frequency shift of 170hz, so
- Omega = 170*2pi?

Part 2.
https://www.youtube.com/watch?v=5GGD99Qi1PA

- alternatively we can calculate for non "90degree shifted" signals:
    Q * sin(2*pi*f*t);
    I * sin(2*pi*f*t + Omega)

    if my theory is correct



Reading form: https://www.allaboutcircuits.com/textbook/radio-frequency-analysis-design/radio-frequency-demodulation/understanding-i-q-signals-and-quadrature-modulation/
Demodulation: https://www.allaboutcircuits.com/textbook/radio-frequency-analysis-design/radio-frequency-demodulation/understanding-quadrature-demodulation/

Finished code: https://github.com/f4exb/sdrangel/blob/master/plugins/channelrx/demodnavtex/readme.md

https://ham.stackexchange.com/questions/5279/comparision-between-bfsk-and-bpsk


Previous thoughts with Omega, while true, are not used in typical IQ.
Normally it's like the first, with sin and cos since the phase shift is 90 degrees.

Regarding WAV: https://stackoverflow.com/questions/13995936/what-is-a-channel-in-a-wav-file-formatdo-all-channels-play-simultaneaously-whe
In each wav sample on a stereo system the data for left vs right amplitude
are separated as LR|LR|LR|LR|LR| where each LR is one frame.
For a 44khz wav file there are 44k LR frames played per second.

Amplitude is sample value. Bit-depth is "how many bits do you use to describe value".

page 59 shows how LR is encoded https://www.mmsp.ece.mcgill.ca/Documents/AudioFormats/WAVE/Docs/riffmci.pdf
for i16 it appear to go between largest and smallest value of i16.

http://whiteboard.ping.se/SDR/IQ
Practice is to represent I as x axis and Q as y axis in 2D diagrams, and I as real part and Q as imaginary part of a complex number.
 It is of no importance if you interchange I and Q, the importance is that they are orthogonal (90°) to each other,
  and using a complex representation is only convenience as well,
  hence no importance if Q is "up" in one graph,
  and I "up" in the next, or which one you happen to represent as the real respectively imaginary part in
   the complex number, if using complex representation at all.


-- So I/Q is nifty. It quite simply means that at one instance of time we can use two amplitudes (I and Q)
    to describe the amplitude, phase AND frequency of the signal they are derived from :) Exactly what
    I was looking for to convert this signal into something readable.

(CARRIER)
    ϕ = phase    = arctan(Q/I)
    Amplitude =(I*I + Q*Q).sqrt()

(BASEBAND?)
Frequency:
    - calculate the phase of your samples k.
    - Do not forget to unwrap ϕ(k) since it is limited in [−π/2,π/2) interval.
    - Take the derivative of ϕ(k) and divide to 2π.
      You can use forward difference technique for this. The result will be your inst. frequency for your I/Q
            (phase(iq_values[x+1]) -phase(iq_values[x])) / 2Pi


In analog demodulation, the signal doesn’t really have a beginning or an end. Imagine an FM transmitter that is broadcasting an audio signal, i.e., a signal that continuously varies according to the music. Now imagine an FM receiver that is initially turned off. The user can power up the receiver at any moment in time, and the demodulation circuitry will begin extracting the audio signal from the modulated carrier. The extracted signal can be amplified and sent to a speaker, and the music will sound normal. The receiver has no idea if the audio signal represents the beginning or end of a song, or if the demodulation circuitry starts functioning at the beginning of a measure, or right on the beat, or in between two beats. It doesn’t matter; each instantaneous voltage value corresponds to one exact moment in the audio signal, and the sound is re-created when all of these instantaneous values occur in succession.

    With digital modulation, the situation is completely different.
    We’re not dealing with instantaneous amplitudes but rather a sequence of amplitudes
     that represents one discrete piece of information, namely, a number (one or zero).
     Each sequence of amplitudes—called a symbol, with a duration equal to one bit period—must
      be distinguished from the preceding and following sequences.
    
    Clearly, then, synchronization must be a priority in any digital RF system.
    One straightforward approach to synchronization is to precede each packet with a predefined
    “training sequence” consisting of alternating zero symbols and one symbols
    (as in the above diagram).
    The receiver can use these one-zero-one-zero transitions to identify the temporal
    boundary between symbols, and then the rest of the symbols in the packet can be
    interpreted properly simply by applying the system’s predefined symbol duration.


    The term “I/Q” is an abbreviation for “in-phase” and “quadrature.”


    The NCO, sometimes called a local oscillator generates digital samples of two sine waves precisely
    offset by 90 degrees in phase creating sine and cosine signals [8], [10], [11], (See Figure 2). It uses a digital
    phase accumulator (adder) and sine/cosine look-up tables. The ADC clock is fed into the local oscillator. The
    digital samples out of the local oscillator are generated at a sampling rate exactly equal to the ADC sample clock
    frequency, fs. Since the data rates from these two mixer input sources are both at the ADC sampling rate, fs, the
    complex mixer output samples at fs. The sine and cosine input from the local oscillator create in-phase and
    quadrature (I and Q) output that are important for maintaining phase information contained in the input signal

    Instantaneous frequency is the derivative of instantenous phase. 

Can likely take inspiration from https://github.com/projecthorus/radiosonde_auto_rx


https://www.youtube.com/watch?v=spUNpyF58BY
https://docs.rs/dft/latest/dft/

Fourier transformations convert a mixed signal into a series of peaks,
The peaks at specific locations tells us that that frequency is inside the mixed signal. 

https://pysdr.org/content/sampling.html


we can convert a signal to the frequency domain using an FFT, and the result is called the Power Spectral Density (PSD). But to actually find the PSD of a batch of samples and plot it, we do more than just take an FFT. We must do the following six operations to calculate PSD:

    * Take the FFT of our samples. If we have x samples, the FFT size will be the length of x by default. Let’s use the first 1024 samples as an example to create a 1024-size FFT. The output will be 1024 complex floats.
    * Take the magnitude of the FFT output, which provides us 1024 real floats.
    * Square the resulting magnitude to get power.
    * Normalize: divide by the FFT size (N) and sample rate (Fs).
    * Convert to dB using 10 \log_{10}(); we always view PSDs in log scale.
    * Perform an FFT shift, covered in the previous chapter, to move “0 Hz” in the center and negative frequencies to the left of center

