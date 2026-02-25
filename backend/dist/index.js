import express from "express";
import "dotenv/config";
const app = express();
const port = process.env.PORT || 3000;
app.get("/", (req, res) => {
    res.send("Blig!");
    console.log("Response sent");
});
app.listen(port, () => {
    console.log(`Example app listening on port ${port}`);
});
//# sourceMappingURL=index.js.map