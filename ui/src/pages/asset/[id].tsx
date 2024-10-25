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

export default function Asset() {
  const router = useRouter();

  const [sharesToBuy, setSharesToBuy] = useState(1);

  const assetData = {
    name: "Luxury Apartment Complex",
    image: "/appartment1.jpg",
    pricePerShare: 100,
    apy: 8.5,
    monthlyIncome: 500,
    category: "Real Estate",
    availableShares: 1000,
  };

  const handleInvest = () => {
    console.log(`Investing in ${sharesToBuy} shares`);
    // Implement investment logic here
  };

  return (
    <div className="bg-white">
      <Navbar />
      <div className="container mx-auto p-4 mt-20">
        <Card className="w-full max-w-4xl mx-auto">
          <CardHeader>
            <CardTitle className="text-2xl font-bold">
              {assetData.name}
            </CardTitle>
          </CardHeader>
          <CardContent className="grid gap-6 md:grid-cols-2">
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
                <div>
                  <p className="font-semibold">Available shares</p>
                  <p>{assetData.availableShares.toLocaleString()}</p>
                </div>
              </div>
            </div>
            <div className="space-y-4">
              <div>
                <Label htmlFor="shares">Number of shares to buy</Label>
                <Input
                  id="shares"
                  type="number"
                  min="1"
                  max={assetData.availableShares}
                  value={sharesToBuy}
                  onChange={(e) => setSharesToBuy(parseInt(e.target.value, 10))}
                  className="mt-1"
                />
              </div>
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
          <CardFooter>
            <Button
              onClick={handleInvest}
              className="w-full bg-black text-white"
            >
              Invest Now
            </Button>
          </CardFooter>
        </Card>
      </div>
    </div>
  );
}
