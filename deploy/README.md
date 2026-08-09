# deploy/

Putting this on a server.

Two forex-specific things that are easy to get wrong:

1. **The server clock must be UTC.** Every candle timestamp is UTC and the
   daily close time is applied deliberately in code. A server on local time
   shifts your daily candles and therefore your levels.

2. **Put the server near your broker.** Not for speed — version 1 places no
   trades — but a distant server drops its connection more often, and every
   dropped connection is a chance to miss candles.
