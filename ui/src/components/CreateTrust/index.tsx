import { useChain } from "@cosmos-kit/react";
import { AnyCnameRecord } from "dns";
import React, { useState } from "react";
import { Coin, StdFee } from "@cosmjs/amino";
import "react-toastify/dist/ReactToastify.css";
import JSONBig from "json-bigint";

const uri =
  "mongodb+srv://helwanmande:jYo6x0ixTaeq4GjR@cluster0.bxgpv.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0";

const CreateTrustForm = () => {
  const chainContext = useChain("mantrachaintestnet2");
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

  const fee: StdFee = {
    amount: [{ denom: "uom", amount: "3594" }],
    gas: "336503",
  };
  const [label, setLabel] = useState("");
  const [instantiateMessage, setInstantiateMessage] = useState("") as any;
  const [instantiateInProcess, setInstantiateInProcess] = useState(false);
  const [instantiateResult, setInstantiateResult] = useState("");

  async function instantiateContract() {
    function stringifyWithBigIntHandling(obj: any) {
      return JSON.stringify(obj, (key, value) =>
        typeof value === "bigint" ? value.toString() : value
      );
    }
    setInstantiateInProcess(true);
    const wasmClient = chainContext.getSigningCosmWasmClient();

    const funds = [{ denom: "uom", amount: "1000" }] as Coin[];
    const parsedMessage = instantiateMessage;

    const result = await wasmClient.then((client) => {
      client
        //@ts-ignore
        .instantiate(address ?? "", 95 ?? 0, parsedMessage, "newTrust", fee, {
          funds,
        })
        .then((result: any) => {
          console.log(result);
          setInstantiateResult(result.contractAddress);
          setInstantiateInProcess(false);

          fetch("/api/mango", {
            method: "POST",
            headers: {
              "Content-Type": "application/json",
            },
            body: JSONBig.stringify({ instantiateMessage, result }),
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
          setInstantiateInProcess(false);
        });
    });
  }

  const [propertyData, setPropertyData] = useState({
    image: null,
    propertyContractCodeId: 96,
    category: "",
    name: "",
    description: "",
    symbol: "",
    imageUrl:
      "https://www.enbdreit.com/-/media/reit/images/home/reit_home_banner_new.jpg?h=618&w=1280&la=en&hash=617DE9DEB355251FF6007568099FFEC4",
  });

  const handleChange = (e: any) => {
    const { name, value, files } = e.target;
    setPropertyData({
      ...propertyData,
      [name]: files ? files[0] : value, // handle file upload for image
    });
  };

  const handleSubmit = (e: AnyCnameRecord) => {
    e.preventDefault();
    // Here you can add your submit logic, like sending data to your backend
    console.log("Property Data:", propertyData);
    const data = {
      admins: [],
      metadata: {
        name: propertyData.name,
        symbol: propertyData.symbol,
        description: propertyData.description,
        image: propertyData.imageUrl,
        category: "real_estate",
      },
      default_royalty_fee: 200,
      property_contract_code_id: null,
    };
    setInstantiateMessage(data);

    instantiateContract();
  };

  return (
    <form className="max-w-2xl mx-auto p-6">
      <p className="mt-10 text-2xl text-center font-bold mb-6">
        Create a Trust
      </p>
      <div className="mb-6">
        <label className="block text-sm font-medium text-gray-700 mb-2">
          Trust Logo
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
            onClick={() => document.querySelector('input[type="file"]').click()}
          >
            Select Image
          </button>
          <p className="text-gray-500 text-sm mt-2">
            Upload an image of the property (max 5MB, .jpg, .png, .webp)
          </p>
        </div>
      </div>

      <div className="grid grid-cols-1 sm:grid-cols-2 gap-6 mb-6">
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
            placeholder="Dubai REIT"
            required
          />
          <p className="text-xs text-gray-500 mt-1">
            The legal name of the trust.
          </p>
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700">
            Symbol
          </label>
          <input
            type="text"
            name="symbol"
            value={propertyData.symbol}
            onChange={handleChange}
            className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
            placeholder="MYTOKEN"
            required
          />
          <p className="text-xs text-gray-500 mt-1">
            The symbol of the governing token of the DAO.
          </p>
        </div>
      </div>

      <div className="grid grid-cols-1 sm:grid-cols-1 gap-6 mb-6">
        <div>
          <label className="block text-sm font-medium text-gray-700">
            Description
          </label>
          <input
            type="text"
            name="description"
            value={propertyData.description}
            onChange={handleChange}
            className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
            placeholder="..."
            required
          />
          <p className="text-xs text-gray-500 mt-1">
            Description of the Trust.
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
          <option value="real_estate">Real Estate</option>
          <option value="real_estate">Commercial</option>
        </select>
        <p className="text-xs text-gray-500 mt-1">
          Select the category that best describes this property.
        </p>
      </div>
      {instantiateResult?.length > 0 && (
        <div className="bg-[#31B47A] my-5 p-3 rounded-full text-white font-bold">
          <p className="text-center">{instantiateResult}</p>
        </div>
      )}

      <button
        onClick={handleSubmit}
        className="w-full bg-black text-white py-3 rounded-md font-medium hover:bg-gray-800 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-black"
      >
        Create Trust
      </button>
    </form>
  );
};

export default CreateTrustForm;
