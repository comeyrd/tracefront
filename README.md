# Tracefront

This is a first prototype to test the capability of Rust and Egui to create an application to display LTE Traces.

This project is linked to [traceserver](https://github.com/comeyrd/traceserver),
It connects to a traceserver, that is a WebSocket server that sends back to any query a JSON formatted like this :
{"direction":"upload","rate":55253,"text":"\u001e","roaming":false}
With each value being randomly selected at each query.

Tracefront displays theses traces in windows that you can open and close.
It compiles to native and to web.

You can find the Tracefront here [https://proj311.ceyraud.com](https://proj311.ceyraud.com)