# TR

## SCX-PARDUS

Pardus için özel olarak geliştirilen, performans ve interaktivite odaklı ve yapay zeka destekli CPU zamanlayıcısı.

sched_ext/scx framework taban alınarak geliştirirmiştir

### Kendi derlediğiniz onnxruntime ile build almak
onnxruntime projesini indirip derleyin: https://github.com/microsoft/onnxruntime

Aşağıdaki komut ile scx-pardus zamanlayıcısını derleyin:
```ORT_LIB_LOCATION=/path/to/onnxruntime/build/Linux/MinSizeRel cargo build -p scx_pardus -r```


# EN

## SCX-PARDUS

AI powered eBPF CPU scheduler designed with performance and interactivity in mind. Made for Pardus operating system. 

Based on sched_ext/scx framework. 

### Building with custom compiled onnxruntime
Clone and build onnxruntime:
https://github.com/microsoft/onnxruntime

Then compile the scx-pardus scheduler:
```ORT_LIB_LOCATION=/path/to/onnxruntime/build/Linux/MinSizeRel cargo build -p scx_pardus -r```



