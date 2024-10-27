// pages/api/instantiate.ts

import type { NextApiRequest, NextApiResponse } from 'next';
import { MongoClient } from 'mongodb';

const uri =  "mongodb+srv://helwanmande:jYo6x0ixTaeq4GjR@cluster0.bxgpv.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0";


export default async function handler(req: NextApiRequest, res: NextApiResponse) {
  // Ensure this is a POST request
  if (req.method !== 'POST') {
    return res.status(405).json({ message: 'Only POST requests are allowed' });
  }

  // Extract instantiateMessage and result (contractAddress) from the request body
  const { instantiateMessage, result } = req.body;

  if (!instantiateMessage || !result) {
    return res.status(400).json({ message: 'Missing instantiateMessage or contractAddress (result)' });
  }

  // Update instantiateMessage with the contractAddress
  instantiateMessage.contractAddress = result;

  try {
    // Connect to the MongoDB database
    const client = await MongoClient.connect(uri, {
    
    });

    const db = client.db('rwa-platform'); // Database name
    const collection = db.collection('trusts'); // Collection name

    // Insert the modified instantiateMessage into MongoDB
    const insertResult = await collection.insertOne(instantiateMessage);

    // Close the MongoDB connection
    client.close();

    // Return success response
    return res.status(200).json({ message: 'Data inserted successfully', insertResult });
  } catch (err) {
    console.error('Error connecting to MongoDB or inserting data:', err);
    //@ts-ignore
    return res.status(500).json({ message: 'Internal Server Error', error: err.message });
  }
}
