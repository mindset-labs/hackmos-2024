//@ts-nocheck

import Navbar from "@/components/Navbar";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/Card";
import { Badge } from "@/components/Badge";
import { useChain } from "@cosmos-kit/react";
import { CwPropertyQueryClient } from "@/utils/protos/cw-property/ts/CwProperty.client";

import {
  DollarSign,
  Percent,
  PiggyBank,
  Tag,
  Wallet,
  UserCheck,
} from "lucide-react";
import { useEffect, useState } from "react";

interface Investment {
  [x: string]: number;
  contractAddress: string;
  name: string;
  sharesOwned: number;
}

const footerNavigation = {
  main: { name: "Hackmos 2024 ❤️", href: "#" },
  social: [
    {
      name: "X",
      href: "https://x.com/TokiFyprotocol",
      icon: (props: any) => (
        <svg fill="currentColor" viewBox="0 0 24 24" {...props}>
          <path d="M13.6823 10.6218L20.2391 3H18.6854L12.9921 9.61788L8.44486 3H3.2002L10.0765 13.0074L3.2002 21H4.75404L10.7663 14.0113L15.5685 21H20.8131L13.6819 10.6218H13.6823ZM11.5541 13.0956L10.8574 12.0991L5.31391 4.16971H7.70053L12.1742 10.5689L12.8709 11.5655L18.6861 19.8835H16.2995L11.5541 13.096V13.0956Z" />
        </svg>
      ),
    },
    {
      name: "GitHub",
      href: "https://github.com/mindset-labs/hackmos-2024",
      icon: (props: any) => (
        <svg fill="currentColor" viewBox="0 0 24 24" {...props}>
          <path
            fillRule="evenodd"
            d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z"
            clipRule="evenodd"
          />
        </svg>
      ),
    },
    {
      name: "Telegram",
      href: "https://t.me/TokiFyProtocol",
      icon: (props: any) => (
        <svg fill="currentColor" viewBox="0 0 24 24" {...props}>
          <svg
            viewBox="0 0 24 24"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
          >
            <g id="SVGRepo_bgCarrier" stroke-width="0"></g>
            <g
              id="SVGRepo_tracerCarrier"
              stroke-linecap="round"
              stroke-linejoin="round"
            ></g>
            <g id="SVGRepo_iconCarrier">
              {" "}
              <path
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M21.997 12C21.997 17.5228 17.5198 22 11.997 22C6.47415 22 1.99699 17.5228 1.99699 12C1.99699 6.47715 6.47415 2 11.997 2C17.5198 2 21.997 6.47715 21.997 12ZM12.3553 9.38244C11.3827 9.787 9.43876 10.6243 6.52356 11.8944C6.05018 12.0827 5.8022 12.2669 5.77962 12.4469C5.74147 12.7513 6.12258 12.8711 6.64155 13.0343C6.71214 13.0565 6.78528 13.0795 6.86026 13.1038C7.37085 13.2698 8.05767 13.464 8.41472 13.4717C8.7386 13.4787 9.10009 13.3452 9.49918 13.0711C12.2229 11.2325 13.629 10.3032 13.7172 10.2831C13.7795 10.269 13.8658 10.2512 13.9243 10.3032C13.9828 10.3552 13.977 10.4536 13.9708 10.48C13.9331 10.641 12.4371 12.0318 11.6629 12.7515C11.4216 12.9759 11.2504 13.135 11.2154 13.1714C11.137 13.2528 11.0571 13.3298 10.9803 13.4038C10.506 13.8611 10.1502 14.204 11 14.764C11.4083 15.0331 11.7351 15.2556 12.0611 15.4776C12.4171 15.7201 12.7722 15.9619 13.2317 16.2631C13.3487 16.3398 13.4605 16.4195 13.5694 16.4971C13.9837 16.7925 14.3559 17.0579 14.8158 17.0155C15.083 16.991 15.359 16.7397 15.4992 15.9903C15.8305 14.2193 16.4817 10.382 16.6322 8.80081C16.6454 8.66228 16.6288 8.48498 16.6154 8.40715C16.6021 8.32932 16.5743 8.21842 16.4731 8.13633C16.3533 8.03911 16.1683 8.01861 16.0856 8.02C15.7095 8.0267 15.1324 8.22735 12.3553 9.38244Z"
                stroke="#000000"
                stroke-linejoin="round"
              ></path>{" "}
            </g>
          </svg>
        </svg>
      ),
    },
  ],
};

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
  const chainContext = useChain("mantrachaintestnet2");

  const { address, connect, isWalletConnected } = chainContext;

  const [propertyShares, setPropertyShares] = useState<Investment[]>([]);
  const [assets, setAssets] = useState([]);

  async function getAllAssets() {
    // Fetch assets from the API
    fetch("/api/getAssets")
      .then((response) => response.json())
      .then((data) => {
        console.log("All Assets:", data.assets);
        setAssets(data.assets); // Set the fetched assets in the state
      })
      .catch((error) => {
        console.error("Error fetching assets:", error);
      });
  }

  async function getPropertyShares() {
    for (let i = 0; i < assets.length; i++) {
      const client = new CwPropertyQueryClient(
        await chainContext.getCosmWasmClient(),
        //@ts-ignore

        assets[i]?.result?.events[9]?.attributes[0]?.value
      );
      console.log(
        "Contract Address",
        //@ts-ignore

        assets[i]?.result?.events[9]?.attributes[0]?.value
      );
      client
        .getShareBalance({
          address: address ?? "",
        })
        .then((shares) => {
          console.log(shares);
          const investment: Investment = {
            //@ts-ignore

            contractAddress: assets[i]?.result?.events[9]?.attributes[0]?.value,
            //@ts-ignore

            name: assets[i].name,
            sharesOwned: Number(shares),
          };
          setPropertyShares((prev) => {
            // Check if this contract address already exists in the array
            const exists = prev.some(
              (item) =>
                item.contractAddress ===
                //@ts-ignore

                assets[i]?.result?.events[9]?.attributes[0]?.value
            );

            if (!exists) {
              // Only add if the investment is not already present
              return [...prev, investment];
            }

            return prev; // If already exists, return the previous state
          });
        });
    }
  }

  async function setUp() {
    await getAllAssets();
    getPropertyShares();
  }

  useEffect(() => {
    setUp();
    console.log("Assets:", assets);
    console.log("Property Shares:", propertyShares);
  }, [isWalletConnected]);

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

              {propertyShares.map((investment) => (
                <div>
                  <h2>{investment.name}</h2>
                  <p>Shares Owned: {investment.sharesOwned}</p>
                  <p>Contract Address: {investment.contractAddress}</p>
                </div>
              ))}

              <h2 className="text-2xl font-bold mb-4">Your Investments</h2>
              <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
                {propertyShares.map((investment) => (
                  <Card className="overflow-hidden">
                    <div className="relative h-38">
                      <img
                        src="https://cdn.properties.emaar.com/wp-content/uploads/2020/03/Grande_Living_Final-5k-opt2-2-scaled.jpg"
                        alt={investment.name}
                      />
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
                        </div>
                        <div className="flex items-center space-x-2">
                          <div className="p-2 bg-blue-100 rounded-full">
                            <Percent className="h-4 w-4 text-blue-600" />
                          </div>
                        </div>
                        <div className="flex items-center space-x-2">
                          <div className="p-2 bg-purple-100 rounded-full">
                            <PiggyBank className="h-4 w-4 text-purple-600" />
                          </div>
                        </div>
                        <div className="flex items-center space-x-2">
                          <div className="p-2 bg-orange-100 rounded-full">
                            <Tag className="h-4 w-4 text-orange-600" />
                          </div>
                        </div>
                      </div>
                      <div className="space-y-2">
                        <div className="flex justify-between text-sm">
                          <span className="font-bold">Shares Owned</span>
                          <span>{investment.sharesOwned}</span>
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
      <footer className="bg-white">
        <div className="mx-auto max-w-7xl overflow-hidden px-6 py-20 sm:py-24 lg:px-8">
          <nav
            aria-label="Footer"
            className="-mb-6 flex flex-wrap justify-center gap-x-12 gap-y-3 text-sm/6"
          >
            <a
              key={footerNavigation.main.name}
              href={footerNavigation.main.href}
              className="text-gray-600 hover:text-gray-900"
            >
              {footerNavigation.main.name}
            </a>
          </nav>
          <div className="mt-16 flex justify-center gap-x-10">
            {footerNavigation.social.map((item) => (
              <a
                key={item.name}
                href={item.href}
                className="text-gray-600 hover:text-gray-800"
              >
                <span className="sr-only">{item.name}</span>
                <item.icon aria-hidden="true" className="h-6 w-6" />
              </a>
            ))}
          </div>
          <p className="mt-10 text-center text-sm/6 text-gray-600">
            &copy; 2024 TokenFi Protocol. All rights reserved.
          </p>
        </div>
      </footer>
    </div>
  );
}
