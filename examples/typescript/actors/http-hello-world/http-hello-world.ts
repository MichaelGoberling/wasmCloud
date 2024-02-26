import {
  IncomingRequest,
  ResponseOutparam,
  OutgoingResponse,
  Fields,
} from "wasi:http/types@0.2.0";

import queryString from 'query-string';

// Implementation of wasi-http incoming-handler
//
// NOTE: To understand the types involved, take a look at wit/deps/http/types.wit
function handle(req: IncomingRequest, resp: ResponseOutparam) {
  // Start building an outgoing response
  const outgoingResponse = new OutgoingResponse(new Fields());

  const pathAndQueryString = req.pathWithQuery() || '/';
  const qs = pathAndQueryString.split('?')[1];
  const parsedQuery = queryString.parse(qs);

  // Access the outgoing response body
  let outgoingBody = outgoingResponse.body();
  // Create a stream for the response body
  let outputStream = outgoingBody.write();
  // // Write hello world to the response stream
  outputStream.blockingWriteAndFlush(
    new Uint8Array(new TextEncoder().encode(`Hello from Typescript! query: ${JSON.stringify(parsedQuery)}\n`))
  );

  // Set the status code for the response
  outgoingResponse.setStatusCode(200);

  // Set the created response
  ResponseOutparam.set(resp, { tag: "ok", val: outgoingResponse });
}

export const incomingHandler = {
  handle,
};
