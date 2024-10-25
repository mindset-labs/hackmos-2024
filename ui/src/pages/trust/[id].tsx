"use-client";

import Navbar from "@/components/Navbar";
import { useRouter } from "next/router";

import { useState } from "react";
import Image from "next/image";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/Card";
import { Input } from "@/components/Input";
import { Button } from "@/components/Button";
import { Label } from "@/components/Label";
import { TokenizedAssetCard } from "@/components/TokenizedCard";
import Link from "next/link";

const properties = [
  {
    title: "IMG_4985.HEIC",
    size: "3.9 MB",
    source: "appartment1.jpg",
    id: 1,
  },
  {
    title: "IMG_4985.HEIC",
    size: "3.9 MB",
    source: "appartment2.jpeg",
    id: 2,
  },
  {
    title: "IMG_4985.HEIC",
    size: "3.9 MB",
    source: "appartment3.jpg",
    id: 3,
  },
];

export default function Trust() {
  const router = useRouter();

  const [sharesToBuy, setSharesToBuy] = useState(1);

  const assetData = {
    name: "Trust Name",
    image: "/trust2.png",
    pricePerShare: 100,
    apy: 8.5,
    monthlyIncome: 500,
    category: "Real Estate",
    availableShares: 1000,
    description: "Trust Description",
  };

  const handleInvest = () => {
    console.log(`Investing in ${sharesToBuy} shares`);
    // Implement investment logic here
  };

  return (
    <div className="bg-white">
      <Navbar />
      <div className="container mx-auto p-4 mt-20">
        <div className="w-full max-w-4xl mx-auto">
          <Button onClick={() => router.back()} className="mb-4">
            Go Back
          </Button>
        </div>
        <Card className="w-full max-w-4xl mx-auto">
          <CardHeader>
            <CardTitle className="text-2xl font-bold">
              {assetData.name}
            </CardTitle>
          </CardHeader>
          <CardContent className="grid gap-6 md:grid-cols-1">
            <div className="space-y-4">
              <Image
                src={assetData.image}
                alt={assetData.name}
                width={400}
                height={300}
                className="w-full rounded-lg object-cover"
              />
              <div className="grid grid-cols-2 gap-4 text-sm">
                <div>
                  <p className="font-semibold">Price per share</p>
                  <p>${assetData.pricePerShare.toFixed(2)}</p>
                </div>
                <div>
                  <p className="font-semibold">APY</p>
                  <p>{assetData.apy}%</p>
                </div>
                <div>
                  <p className="font-semibold">Monthly income</p>
                  <p>${assetData.monthlyIncome.toFixed(2)}</p>
                </div>
                <div>
                  <p className="font-semibold">Category</p>
                  <p>{assetData.category}</p>
                </div>
              </div>
            </div>
            <div>
              <p className="font-semibold">Description</p>
              <p>{assetData.description}</p>
            </div>
            <div className="space-y-4">
              <div>
                <p className="text-lg font-semibold">
                  Total Investment: $
                  {(sharesToBuy * assetData.pricePerShare).toFixed(2)}
                </p>
                <p className="text-sm text-muted-foreground">
                  Estimated Monthly Income: $
                  {(
                    (sharesToBuy * assetData.monthlyIncome) /
                    assetData.availableShares
                  ).toFixed(2)}
                </p>
              </div>
            </div>
          </CardContent>
          <CardFooter></CardFooter>
        </Card>
      </div>
      <div className="mb-20">
        <p className="text-center pb-10 mt-2 text-pretty text-2xl font-semibold tracking-tight text-gray-900 sm:text-2xl">
          Assets under management
        </p>
        <div className="flex justify-center">
          <ul role="list" className="grid grid-cols-3 gap-x-4">
            {properties.map((file) => (
              <Link href={`asset/${file.id}`}>
                <TokenizedAssetCard
                  imageUrl="/appartment1.jpg"
                  title="Premium Office Space"
                  pricePerShare={250}
                  apy={7.5}
                  monthlyIncome={1200}
                  category="Commercial Real Estate"
                />
              </Link>
            ))}
          </ul>
        </div>
      </div>
    </div>
  );
}
