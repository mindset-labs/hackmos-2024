// pages/api/assets.ts
import { NextApiRequest, NextApiResponse } from 'next';
import { MongoClient } from 'mongodb';

const uri =  "mongodb+srv://helwanmande:jYo6x0ixTaeq4GjR@cluster0.bxgpv.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0";

export default async function handler(req: NextApiRequest, res: NextApiResponse) {
  if (req.method !== 'GET') {
    return res.status(405).json({ message: 'Only GET requests are allowed' });
  }

  try {
    const client = await MongoClient.connect(uri, {
    
    });

    const db = client.db('rwa-platform');
    const collection = db.collection('assets'); // Assuming you have a "trusts" collection

    const assets = await collection.find({}).toArray(); // Fetch all assets

    client.close();

    return res.status(200).json({ assets: assets });
  } catch (err) {
    console.error('Error fetching assets:', err);
    return res.status(500).json({ message: 'Error fetching assets', error: err.message });
  }
}
