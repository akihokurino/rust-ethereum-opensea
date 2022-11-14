import express from "express";

const app = express();

app.use(express.json());
app.use(express.urlencoded({ extended: true }));

app.post("/", (req: express.Request, res: express.Response) => {
  const now = new Date();
  res.send(
    JSON.stringify({
      data: {
        jobRunID: req.body.id ?? 0,
        now: now.toISOString(),
        timestamp: now.getTime(),
      },
    })
  );
});

const port = 8080;
app.listen(port, () => {
  console.log("Node.js is listening to PORT:" + port);
});
