// @ts-nocheck
import { useChain } from "@cosmos-kit/react";
import React, { useEffect, useState } from "react";
import { Coin, StdFee } from "@cosmjs/amino";
import { CwDaoClient } from "@/utils/protos/ cw-dao/ts/CwDao.client";
import { DAOProperty } from "@/utils/protos/ cw-dao/ts/CwDao.types";
import JSONBig from "json-bigint";

const ListPropertyForm = () => {
  const chainContext = useChain("neutrontestnet");
  const [contractAddress, setContractAddress] = useState("");
  const [isAssetCreated, setIsAssetCreated] = useState(false);

  const fee: StdFee = {
    amount: [{ denom: "untrn", amount: "3594" }],
    gas: "497883",
  };

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

  const [propertyData, setPropertyData] = useState({
    image: null,
    monthlyIncome: "",
    pricePerShare: "",
    numberOfShares: "",
    apy: "",
    category: "",
    contractAddress: "",
    name: "",
    total_shares: "",
    subcategory: "",
    symbol: "",
    royalty_fee: "",
  });

  const handleChange = (e: any) => {
    const { name, value, files } = e.target;
    setPropertyData({
      ...propertyData,
      [name]: files ? files[0] : value, // handle file upload for image
    });
  };

  const handleSubmit = (e: any) => {
    e.preventDefault();
    // Here you can add your submit logic, like sending data to your backend
    console.log("Property Data:", propertyData);

    createProperty();
  };

  const [trusts, setTrusts] = useState([]);
  const [selectedTrust, setSelectedTrust] = useState();

  const handleTrustChange = (e: any) => {
    setSelectedTrust(e.target.value); // Update the selectedTrust state
    console.log("Selected Trust:", e.target.value);
    console.log(contractAddress);
    setContractAddress(e.target.value);
  };
  function getAllTrusts() {
    fetch("/api/getTrusts")
      .then((response) => response.json())
      .then((data) => {
        console.log("All Trusts:", data.trusts);
        setTrusts(data.trusts);
      })
      .catch((error) => {
        console.error("Error fetching trusts:", error);
      });
  }

  useEffect(() => {
    getAllTrusts();
  }, []);

  const funds = [{ denom: "untrn", amount: "1000" }] as Coin[];
  // const parsedMessage = instantiateMessage;

  async function createProperty() {
    const executeClient = new CwDaoClient(
      await chainContext.getSigningCosmWasmClient(),
      address ?? "",
      contractAddress
    );

    const properties: DAOProperty = {
      estimated_apy: Number(propertyData.apy),
      estimated_monthly_income: {
        amount: propertyData.monthlyIncome,
        denom: "untrn",
      },
      image_uri:
        "https://cdn.properties.emaar.com/wp-content/uploads/2020/03/Grande_Living_Final-5k-opt2-2-scaled.jpg",
      name: propertyData.name,
      price_per_share: { amount: propertyData.pricePerShare, denom: "untrn" },
      royalty_fee: Number(propertyData.royalty_fee),
      status: "Active",
      subcategory: propertyData.category,
      symbol: propertyData.symbol,
      total_shares: Number(propertyData.numberOfShares),
    };

    executeClient
      .launchProperty({ data: properties }, fee)
      .then((result) => {
        console.log("Property Created:", result);
        setIsAssetCreated(true);
        fetch("/api/listAsset", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSONBig.stringify({ properties, result }),
        })
          .then((response) => response.json())
          .then((data) => {
            console.log("Success:", data);
          })
          .catch((error) => {
            console.error("Error:", error);
          });
      })
      .catch((e) => {
        console.error(e);
      });
  }

  return (
    <form className="max-w-2xl mx-auto p-6">
      <p className="mt-10 text-2xl text-center font-bold mb-6">
        List a New Property
      </p>
      <div className="mb-6">
        <label className="block text-sm font-medium text-gray-700 mb-2">
          Property Image
        </label>
        <div className="flex flex-col items-center justify-center border-2 border-dashed border-gray-300 rounded-lg p-6">
          <input
            type="file"
            name="image"
            accept="image/*"
            onChange={handleChange}
            className="hidden"
          />
          <button
            type="button"
            className="bg-blue-500 text-white py-2 px-4 rounded-md"
            //@ts-ignore
            onClick={() => document.querySelector('input[type="file"]').click()}
          >
            Select Image
          </button>
          <p className="text-gray-500 text-sm mt-2">
            Upload an image of the property (max 5MB, .jpg, .png, .webp)
          </p>
        </div>
      </div>
      <div>
        <label className="block text-sm font-medium text-gray-700">
          Select Trust
        </label>
        <select
          name="selectedTrust"
          value={selectedTrust} // Use the selectedTrust state here
          onChange={handleTrustChange} // Handle trust selection
          className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
          required
        >
          <option value="">Select a trust</option>
          {trusts.map((trust) => (
            //@ts-ignore

            <option key={trust.metadata.name} value={trust[0]}>
              {trust.contractAddress.contractAddress}
            </option>
          ))}
        </select>
        <p className="text-xs text-gray-500 mt-1">
          Select a trust from the list of available trusts.
        </p>
      </div>

      <div className="grid grid-cols-1 sm:grid-cols-2 gap-6 mb-6">
        <div>
          <label className="block text-sm font-medium text-gray-700">
            Monthly Income
          </label>
          <input
            type="number"
            name="monthlyIncome"
            value={propertyData.monthlyIncome}
            onChange={handleChange}
            className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
            placeholder="5000"
            required
          />
          <p className="text-xs text-gray-500 mt-1">
            The expected monthly income from this property.
          </p>
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700">
            Name
          </label>
          <input
            type="text"
            name="name"
            value={propertyData.name}
            onChange={handleChange}
            className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
            placeholder="My Property"
            required
          />
          <p className="text-xs text-gray-500 mt-1">
            The legal name of the asset.
          </p>
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700">
            Price per Share
          </label>
          <input
            type="number"
            name="pricePerShare"
            value={propertyData.pricePerShare}
            onChange={handleChange}
            className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
            placeholder="100"
            required
          />
          <p className="text-xs text-gray-500 mt-1">
            The price for a single share of this property.
          </p>
        </div>
      </div>

      <div className="grid grid-cols-1 sm:grid-cols-2 gap-6 mb-6">
        <div>
          <label className="block text-sm font-medium text-gray-700">
            Number of Shares Available
          </label>
          <input
            type="number"
            name="numberOfShares"
            value={propertyData.numberOfShares}
            onChange={handleChange}
            className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
            placeholder="1000"
            required
          />
          <p className="text-xs text-gray-500 mt-1">
            The total number of shares available for this property.
          </p>
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700">
            APY (Annual Percentage Yield)
          </label>
          <input
            type="number"
            name="apy"
            value={propertyData.apy}
            onChange={handleChange}
            className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
            placeholder="5.5"
            required
          />
          <p className="text-xs text-gray-500 mt-1">
            The expected annual percentage yield for investors.
          </p>
        </div>
      </div>

      <div className="mb-6">
        <label className="block text-sm font-medium text-gray-700">
          Category
        </label>
        <select
          name="category"
          value={propertyData.category}
          onChange={handleChange}
          className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
          required
        >
          <option value="">Select a category</option>
          <option value="Residential">Residential</option>
          <option value="Commercial">Commercial</option>
        </select>
        <p className="text-xs text-gray-500 mt-1">
          Select the category that best describes this property.
        </p>
      </div>
      {isAssetCreated && (
        <div className="bg-[#31B47A] my-5 p-3 rounded-full text-white font-bold">
          <p className="text-center">Asset created succesfully</p>
        </div>
      )}

      <button
        onClick={handleSubmit}
        className="w-full bg-black text-white py-3 rounded-md font-medium hover:bg-gray-800 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-black"
      >
        List Property
      </button>
    </form>
  );
};

export default ListPropertyForm;
