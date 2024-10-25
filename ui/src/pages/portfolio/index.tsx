import Navbar from "@/components/Navbar";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/Card";
import { Progress } from "@/components/Progress";
import { Badge } from "@/components/Badge";
import { useChain } from "@cosmos-kit/react";

import {
  DollarSign,
  Percent,
  PiggyBank,
  Tag,
  Wallet,
  UserCheck,
} from "lucide-react";

const user = {
  name: "John Doe",
  kycStatus: "Verified",
  walletAddress: "0x1234...5678",
  monthlyIncome: 2500,
  investments: [
    {
      id: 1,
      name: "Premium Office Space",
      sharesOwned: 100,
      totalShares: 1000,
      monthlyIncome: 1200,
      pricePerShare: 250,
      apy: 7.5,
      category: "Commercial Real Estate",
      image: "appartment1.jpg",
    },
    {
      id: 2,
      name: "Luxury Apartment Complex",
      sharesOwned: 50,
      totalShares: 500,
      monthlyIncome: 800,
      pricePerShare: 300,
      apy: 6.8,
      category: "Residential Real Estate",
      image: "appartment1.jpg",
    },
    {
      id: 3,
      name: "Retail Shopping Center",
      sharesOwned: 200,
      totalShares: 2000,
      monthlyIncome: 500,
      pricePerShare: 150,
      apy: 8.2,
      category: "Commercial Real Estate",
      image: "appartment1.jpg",
    },
  ],
};

export default function Portfolio() {
  const chainContext = useChain("neutron");

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

  return (
    <div className="bg-white">
      <Navbar />
      <div className="mt-24">
        <div className="container mx-auto p-4">
          <h1 className="text-3xl font-bold mb-6 text-center">
            Your Portfolio
          </h1>

          {isWalletConnected ? (
            <div>
              <div className="grid gap-6 md:grid-cols-3 mb-8">
                <Card>
                  <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                    <CardTitle className="text-sm font-medium">
                      Total Monthly Income
                    </CardTitle>
                    <PiggyBank className="h-4 w-4 text-purple-600" />
                  </CardHeader>
                  <CardContent>
                    <div className="text-2xl font-bold">
                      ${user.monthlyIncome.toFixed(2)}
                    </div>
                  </CardContent>
                </Card>
                <Card>
                  <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                    <CardTitle className="text-sm font-medium">
                      KYC Status
                    </CardTitle>
                    <UserCheck className="h-4 w-4 text-green-600" />
                  </CardHeader>
                  <CardContent>
                    <Badge variant="default" className="text-lg">
                      {user.kycStatus}
                    </Badge>
                  </CardContent>
                </Card>
                <Card>
                  <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                    <CardTitle className="text-sm font-medium">
                      Wallet Address
                    </CardTitle>
                    <Wallet className="h-4 w-4 text-blue-600" />
                  </CardHeader>
                  <CardContent>
                    <div className="text-xl font-mono">
                      {address?.slice(0, address.length - 30)}...
                      {address?.slice(-4)}
                    </div>
                  </CardContent>
                </Card>
              </div>

              <h2 className="text-2xl font-bold mb-4">Your Investments</h2>
              <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
                {user.investments.map((investment) => (
                  <Card key={investment.id} className="overflow-hidden">
                    <div className="relative h-48">
                      <img src={investment.image} alt={investment.name} />
                    </div>
                    <CardHeader>
                      <CardTitle>{investment.name}</CardTitle>
                    </CardHeader>
                    <CardContent>
                      <div className="grid grid-cols-2 gap-4 mb-4">
                        <div className="flex items-center space-x-2">
                          <div className="p-2 bg-green-100 rounded-full">
                            <DollarSign className="h-4 w-4 text-green-600" />
                          </div>
                          <div>
                            <p className="text-xs text-muted-foreground">
                              Price per Share
                            </p>
                            <p className="font-semibold">
                              ${investment.pricePerShare.toFixed(2)}
                            </p>
                          </div>
                        </div>
                        <div className="flex items-center space-x-2">
                          <div className="p-2 bg-blue-100 rounded-full">
                            <Percent className="h-4 w-4 text-blue-600" />
                          </div>
                          <div>
                            <p className="text-xs text-muted-foreground">APY</p>
                            <p className="font-semibold">{investment.apy}%</p>
                          </div>
                        </div>
                        <div className="flex items-center space-x-2">
                          <div className="p-2 bg-purple-100 rounded-full">
                            <PiggyBank className="h-4 w-4 text-purple-600" />
                          </div>
                          <div>
                            <p className="text-xs text-muted-foreground">
                              Monthly Income
                            </p>
                            <p className="font-semibold">
                              ${investment.monthlyIncome.toFixed(2)}
                            </p>
                          </div>
                        </div>
                        <div className="flex items-center space-x-2">
                          <div className="p-2 bg-orange-100 rounded-full">
                            <Tag className="h-4 w-4 text-orange-600" />
                          </div>
                          <div>
                            <p className="text-xs text-muted-foreground">
                              Category
                            </p>
                            <p className="font-semibold">
                              {investment.category}
                            </p>
                          </div>
                        </div>
                      </div>
                      <div className="space-y-2">
                        <div className="flex justify-between text-sm">
                          <span>Shares Owned</span>
                          <span>
                            {investment.sharesOwned} / {investment.totalShares}
                          </span>
                        </div>
                        {/* <Progress
                    value={
                      (investment.sharesOwned / investment.totalShares) * 100
                    }
                    className="h-2"
                  /> */}
                        <div className="text-xs text-right text-muted-foreground">
                          {(
                            (investment.sharesOwned / investment.totalShares) *
                            100
                          ).toFixed(2)}
                          %
                        </div>
                      </div>
                      {/* <div className="mt-4 aspect-square">
                  <ChartWrapper
                    content={Chart1}
                    className="w-full h-full"
                    title={`Share Distribution for ${investment.name}`}
                    data={[
                      { name: "Your Shares", value: investment.sharesOwned },
                      {
                        name: "Other Shares",
                        value: investment.totalShares - investment.sharesOwned,
                      },
                    ]}
                  />
                </div> */}
                    </CardContent>
                  </Card>
                ))}
              </div>
            </div>
          ) : (
            <div className="text-center">
              <button
                onClick={connect}
                className="bg-blue-500 hover:bg-blue-600 text-white font-semibold py-2 px-4 rounded-lg"
              >
                Connect Wallet
              </button>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
