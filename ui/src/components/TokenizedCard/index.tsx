import { Card, CardContent, CardHeader, CardTitle } from "@/components/Card";
import { Badge } from "@/components/Badge";
import { DollarSign, Percent, PiggyBank, Tag } from "lucide-react";
import Image from "next/image";

export function TokenizedAssetCard({
  imageUrl = "/placeholder.svg?height=200&width=300",
  title = "Tokenized Asset",
  pricePerShare = 100,
  apy = 5.2,
  monthlyIncome = 500,
  category = "Real Estate",
}: {
  imageUrl?: string;
  title?: string;
  pricePerShare?: number;
  apy?: number;
  monthlyIncome?: number;
  category?: string;
}) {
  return (
    <Card className="w-full max-w-sm overflow-hidden">
      <CardHeader className="p-0">
        <Image
          src={imageUrl}
          alt={title}
          width={300}
          height={200}
          className="w-full h-48 object-cover"
        />
      </CardHeader>
      <CardContent className="p-4">
        <CardTitle className="text-xl mb-4">{title}</CardTitle>
        <div className="grid grid-cols-2 gap-4">
          <div className="flex items-center space-x-2">
            <DollarSign className="w-5 h-5 text-green-500" />
            <div>
              <p className="text-sm font-medium">Price per Share</p>
              <p className="text-lg font-bold">${pricePerShare.toFixed(2)}</p>
            </div>
          </div>
          <div className="flex items-center space-x-2">
            <Percent className="w-5 h-5 text-blue-500" />
            <div>
              <p className="text-sm font-medium">APY</p>
              <p className="text-lg font-bold">{apy.toFixed(1)}%</p>
            </div>
          </div>
          <div className="flex items-center space-x-2">
            <PiggyBank className="w-5 h-5 text-purple-500" />
            <div>
              <p className="text-sm font-medium">Monthly Income</p>
              <p className="text-lg font-bold">${monthlyIncome.toFixed(2)}</p>
            </div>
          </div>
          <div className="flex items-center space-x-2">
            <Tag className="w-5 h-5 text-orange-500" />
            <div>
              <p className="text-sm font-medium">Category</p>
              <Badge variant="secondary" className="mt-1">
                {category}
              </Badge>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
