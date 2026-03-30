export const handler = async (_event: unknown, _context: unknown) => {
  return {
    statusCode: 200,
    body: JSON.stringify('Hello, World!'),
  };
};
