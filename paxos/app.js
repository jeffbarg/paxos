const express = require('express')
const bodyParser = require('body-parser')
const crypto = require('crypto')

const app = express()
const port = 3000

app.use(bodyParser.json());

// Currently store all state in one global object. Obviously, this is not scalable or persisted.
// To fix persistance, we could switch to a persisted in-memory store such as LevelDB. 
// If scaling performance is an issue, this could be scaled up by using a low-latency datastore such as DynamoDB or Redis 
// on seperate host(s).
digests = {}

// Allow users to post messages to the message endpoint
app.post('/messages', function (req, res) {
	let message = req.body.message
	if (message === undefined) {
        res.status(400).send({ error: 'Message was not provided in the request' });
        return
	}

	const digest = crypto.createHash('sha256')
					   .update(message)
	                   .digest('hex');
	digests[digest] = message

  	res.send({digest})
})

// Allow users to retrieve messages by previously calculated SHA256 hashes
app.get('/messages/:digest', function (req, res) {
	let digest = req.params.digest
	let message = digests[digest]

	if (digest === undefined) {
        res.status(400).send({ error: 'Digest was not provided in the request' });
        return
	}
	if (message === undefined) {
        res.status(404).send({ error: 'No message corresponds to this digest' });
        return
	}

	res.send({message})
})

// Default 404
app.use(function (req, res, next) {
  res.status(404).send("Sorry can't find that!")
})

// Start server
app.listen(port, () => console.log(`Paxos app listening on port ${port}!`))