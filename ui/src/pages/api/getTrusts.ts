// pages/api/getTrusts.ts

import type { NextApiRequest, NextApiResponse } from 'next';
import { MongoClient } from 'mongodb';

const uri =  "mongodb+srv://helwanmande:jYo6x0ixTaeq4GjR@cluster0.bxgpv.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0";

export default async function handler(req: NextApiRequest, res: NextApiResponse) {
  // Ensure this is a GET request
  if (req.method !== 'GET') {
    return res.status(405).json({ message: 'Only GET requests are allowed' });
  }

  try {
    // Connect to the MongoDB database
    const client = await MongoClient.connect(uri, {
    
    });

    const db = client.db('rwa-platform'); // Database name
    const collection = db.collection('trusts'); // Collection name

    // Fetch all documents from the collection
    const allTrusts = await collection.find({}).toArray();

    // Close the MongoDB connection
    client.close();

    // Return the data
    return res.status(200).json({ trusts: allTrusts });
  } catch (err) {
    console.error('Error fetching data from MongoDB:', err);
    return res.status(500).json({ message: 'Internal Server Error', error: err.message });
  }
}
