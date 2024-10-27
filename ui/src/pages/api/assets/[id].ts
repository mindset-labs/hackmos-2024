// pages/api/assets/[id].ts
// @ts-nocheck
import { NextApiRequest, NextApiResponse } from 'next';
import { MongoClient, ObjectId } from 'mongodb';

const uri =  "mongodb+srv://helwanmande:jYo6x0ixTaeq4GjR@cluster0.bxgpv.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0";

export default async function handler(req: NextApiRequest, res: NextApiResponse) {
  // Only allow GET requests
  if (req.method !== 'GET') {
    return res.status(405).json({ message: 'Only GET requests are allowed' });
  }

  const { id } = req.query; // Get the asset ID from the query params

  if (!ObjectId.isValid(id as string)) {
    return res.status(400).json({ message: 'Invalid asset ID' });
  }

  try {
    const client = await MongoClient.connect(uri, {

    });

    const db = client.db('rwa-platform'); // Replace with your DB name
    const collection = db.collection('assets'); // Replace with your collection name

    // Fetch the asset by its ID
    const asset = await collection.findOne({ _id: new ObjectId(id as string) });

    client.close();

    if (!asset) {
      return res.status(404).json({ message: 'Asset not found' });
    }

    return res.status(200).json(asset);
  } catch (err) {
    console.error('Error fetching asset:', err);
    return res.status(500).json({ message: 'Internal Server Error', error: err.message });
  }
}
