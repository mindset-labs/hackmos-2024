//@ts-nocheck

"use-client";

import Navbar from "@/components/Navbar";
import { useRouter } from "next/router";

import { useEffect, useState } from "react";
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
import { useChain } from "@cosmos-kit/react";
import { CwPropertyClient } from "@/utils/protos/cw-property/ts/CwProperty.client";
import { Coin } from "@/utils/protos/ cw-dao/ts/CwDao.types";

export default function Asset() {
  const router = useRouter();
  const id = router.query.id;
  const [investResponse, setInvestResponse] = useState(null);

  const chainContext = useChain("neutrontestnet");
  const {
    status,
    username,
    address,
    message,
    connect,
    disconnect,
    openView,
    isWalletConnected,
  } = chainContext;

  const [sharesToBuy, setSharesToBuy] = useState(1);

  const [assetData, setAssetData] = useState({
    name: "Luxury Apartment Complex",
    image: "/appartment1.jpg",
    pricePerShare: 100,
    apy: 8.5,
    monthlyIncome: 500,
    category: "Real Estate",
    availableShares: 1000,
  });

  const handleInvest = () => {
    console.log(`Investing in ${sharesToBuy} shares`);
    // Implement investment logic here
    invest();
  };

  useEffect(() => {
    if (router.isReady) {
      console.log("trying to fetch this id", router.query.id);
      // Fetch asset by ID
      fetch(`/api/assets/${id}`)
        .then((response) => {
          if (!response.ok) {
            throw new Error("Asset not found");
          }
          console.log({ response });
          return response.json();
        })
        .then((data) => {
          setAssetData(data);
        })
        .catch((err) => {
          console.log(err.message);
        });
    }
  }, [router.isReady, router.query]);

  async function invest() {
    const executeClient = new CwPropertyClient(
      await chainContext.getSigningCosmWasmClient(),
      address ?? "",
      assetData?.result?.events[9]?.attributes[0]?.value
    );

    const fee: StdFee = {
      amount: [{ denom: "untrn", amount: "3594" }],
      gas: "497883",
    };

    const funds: Coin = {
      denom: "untrn",
      amount: "1000",
    };

    executeClient
      .buyShares(
        {
          amount: sharesToBuy.toString(),
        },
        fee,
        "",
        [funds]
      )
      .then((response) => {
        console.log(response);
        setInvestResponse(response);
      });
  }

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
              {assetData?.name}
            </CardTitle>
            <p>
              <span className="font-bold">Address:</span>
              {""}
              {assetData?.result?.events[9]?.attributes[0]?.value}
            </p>
            {investResponse && (
              <div className="bg-green-500 p-4 rounded-full">
                <p className="text-sm ">
                  {investResponse ? investResponse?.transactionHash : ""}
                </p>
              </div>
            )}
          </CardHeader>
          <CardContent className="grid gap-6 md:grid-cols-2">
            <div className="space-y-4">
              <img
                src={assetData?.image_uri}
                alt={assetData?.name}
                width={400}
                height={300}
                className="w-full rounded-lg object-cover"
              />
              <div className="grid grid-cols-2 gap-4 text-sm">
                <div>
                  <p className="font-semibold">Price per share</p>
                  <p>${assetData?.price_per_share?.amount}</p>
                </div>
                <div>
                  <p className="font-semibold">APY</p>
                  <p>{assetData?.estimated_apy}%</p>
                </div>
                <div>
                  <p className="font-semibold">Monthly income</p>
                  <p>${assetData?.estimated_monthly_income?.amount}</p>
                </div>
                <div>
                  <p className="font-semibold">Category</p>
                  <p>{assetData?.subcategory}</p>
                </div>
                <div>
                  <p className="font-semibold">Available shares</p>
                  <p>{assetData?.total_shares}</p>
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
                  {sharesToBuy * assetData?.price_per_share?.amount}
                </p>
                <p className="text-sm text-muted-foreground">
                  Estimated Monthly Income: $
                  {(sharesToBuy * assetData?.estimated_monthly_income?.amount) /
                    assetData.total_shares}
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
