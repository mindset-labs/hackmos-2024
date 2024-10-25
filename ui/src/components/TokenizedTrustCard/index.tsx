import { Card, CardContent, CardHeader, CardTitle } from "@/components/Card";
import { Badge } from "@/components/Badge";
import { Building, Tag, Home, Wallet, Users, Percent } from "lucide-react";
import Image from "next/image";

export default function TokenizeTrustCard({
  imageUrl = "/placeholder.svg?height=200&width=300",
  name = "Tokenized Trust",
  category = "Real Estate",
  numberOfProperties = 10,
  portfolioValue = 1000000,
  numberOfInvestors = 500,
  apy = 6.5,
}: {
  imageUrl?: string;
  name?: string;
  category?: string;
  numberOfProperties?: number;
  portfolioValue?: number;
  numberOfInvestors?: number;
  apy?: number;
}) {
  return (
    <Card className="w-full max-w-sm overflow-hidden">
      <CardHeader className="p-0">
        <Image
          src={imageUrl}
          alt={name}
          width={300}
          height={200}
          className="w-full h-48 object-cover"
        />
      </CardHeader>
      <CardContent className="p-4">
        <CardTitle className="text-xl mb-2">{name}</CardTitle>
        <Badge variant="secondary" className="mb-4">
          {category}
        </Badge>
        <div className="grid grid-cols-2 gap-4">
          <div className="flex items-center space-x-2">
            <Home className="w-5 h-5 text-blue-500" />
            <div>
              <p className="text-sm font-medium">Properties</p>
              <p className="text-lg font-bold">{numberOfProperties}</p>
            </div>
          </div>
          <div className="flex items-center space-x-2">
            <Wallet className="w-5 h-5 text-green-500" />
            <div>
              <p className="text-sm font-medium">Portfolio Value</p>
              <p className="text-lg font-bold">
                ${portfolioValue.toLocaleString()}
              </p>
            </div>
          </div>
          <div className="flex items-center space-x-2">
            <Users className="w-5 h-5 text-purple-500" />
            <div>
              <p className="text-sm font-medium">Investors</p>
              <p className="text-lg font-bold">
                {numberOfInvestors.toLocaleString()}
              </p>
            </div>
          </div>
          <div className="flex items-center space-x-2">
            <Percent className="w-5 h-5 text-orange-500" />
            <div>
              <p className="text-sm font-medium">APY</p>
              <p className="text-lg font-bold">{apy.toFixed(1)}%</p>
            </div>
          </div>
        </div>
      </CardContent>
      <button className="block w-full py-2 text-center bg-black text-white font-semibold hover:bg-white hover:text-black">
        Preview
      </button>
    </Card>
  );
}
