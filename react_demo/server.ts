import { serveDir } from "https://deno.land/std@0.220.1/http/file_server.ts";

Deno.serve((req) => {
  return serveDir(req, {
    fsRoot: ".",
  });
});
