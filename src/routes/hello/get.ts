globalThis.handler = function(req) {
  return `Hello, ${req.query.name} !`;
};