//@ts-nocheck

"use client";

import Navbar from "@/components/Navbar";
import {
  QueueListIcon,
  WindowIcon,
  ScaleIcon,
} from "@heroicons/react/20/solid";
import { Card, CardHeader, CardBody, Image, Link } from "@nextui-org/react";
import { TokenizedAssetCard } from "@/components/TokenizedCard";
import TokenizeTrustCard from "@/components/TokenizedTrustCard";
import { useEffect, useState } from "react";
import { useChain } from "@cosmos-kit/react";

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

const features = [
  {
    name: "Fractionalized Asset Ownership.",
    description:
      "TOKIFY increases accessibility for a broader range of investors by enabling fractional ownership of real estate assets, allowing individuals to participate in the market with smaller investment amounts.",
    icon: QueueListIcon,
  },
  {
    name: "Comprehensive Legal Framework.",
    description:
      "TOKIFY offers a robust legal framework that is fully compliant with regulations in the Dubai International Financial Center (DIFC), ensuring accessibility for investors and providing regulatory certainty.",
    icon: ScaleIcon,
  },
  {
    name: "User-Friendly Interface.",
    description:
      "TOKIFY allows users to easily create new Real Estate Investment Trusts (REITs) and add real estate, making it accessible for all investors and real estate owners.",
    icon: WindowIcon,
  },
];

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

export default function Example() {
  const chainContext = useChain("neutrontestnet");

  const [queryMessageResultTrust, setQueryMessageResultTrust] = useState("");
  const [trusts, setTrusts] = useState([]);
  const [assets, setAssets] = useState([]);

  const [queryMessageInProcess, setQueryMessageInProcess] = useState(false);
  const queryContract = async () => {
    setQueryMessageInProcess(true);
    const wasmClient = chainContext.getSigningCosmWasmClient();
    const queryDAO = {
      get_metadata: {},
    };
    const parsedMessage = queryDAO;

    await wasmClient.then((client) => {
      client
        .queryContractSmart(
          "mantra1z87qy8fdv2fcq8mdh6hn9ftv42lvmtear7q0gp4wg2w9fdkjfsgsszqn4q",
          parsedMessage
        )
        .then((result: any) => {
          console.log(result);
          setQueryMessageResultTrust(result);
          setQueryMessageInProcess(false);
        })
        .catch((e) => {
          console.error(e);
          setQueryMessageInProcess(false);
        });
    });
  };

  useEffect(() => {
    if (chainContext.isWalletConnected) {
      queryContract();
    }
  }, [chainContext.isWalletConnected]);

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

  function getAllAssets() {
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

  useEffect(() => {
    getAllTrusts();
    getAllAssets();
    console.log("Trusts:", trusts);
    console.log("Assets:", assets);
  }, []);

  return (
    <div className="bg-white">
      <Navbar />
      <div className="relative isolate px-6 pt-14 lg:px-8">
        <div
          aria-hidden="true"
          className="absolute inset-x-0 -top-40 -z-10 transform-gpu overflow-hidden blur-3xl sm:-top-80"
        >
          <div
            style={{
              clipPath:
                "polygon(74.1% 44.1%, 100% 61.6%, 97.5% 26.9%, 85.5% 0.1%, 80.7% 2%, 72.5% 32.5%, 60.2% 62.4%, 52.4% 68.1%, 47.5% 58.3%, 45.2% 34.5%, 27.5% 76.7%, 0.1% 64.9%, 17.9% 100%, 27.6% 76.8%, 76.1% 97.7%, 74.1% 44.1%)",
            }}
            className="relative left-[calc(50%-11rem)] aspect-[1155/678] w-[36.125rem] -translate-x-1/2 rotate-[30deg] bg-gradient-to-tr from-[#ff80b5] to-[#9089fc] opacity-30 sm:left-[calc(50%-30rem)] sm:w-[72.1875rem]"
          />
        </div>
        <div className="mx-auto max-w-2xl py-32 sm:py-48 lg:py-56">
          <div className="text-center">
            <h1 className="text-balance text-5xl font-semibold tracking-tight text-gray-900 sm:text-7xl">
              Tokenize, Earn, Trade
            </h1>
            <p className="mt-8 text-pretty text-lg font-medium text-gray-500 sm:text-xl/8">
              Tokify allows you to tokenize your assets, earn passive income by
              selling shares, and trade on the open market.
            </p>
            <div className="mt-10 flex items-center justify-center gap-x-6">
              <a
                href="#"
                className="rounded-md bg-black px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
              >
                Create a Trust
              </a>
              <a
                href="#"
                className="text-sm font-semibold leading-6 text-gray-900"
              >
                List an asset <span aria-hidden="true">→</span>
              </a>
            </div>
          </div>
        </div>
        <div
          aria-hidden="true"
          className="absolute inset-x-0 top-[calc(100%-13rem)] -z-10 transform-gpu overflow-hidden blur-3xl sm:top-[calc(100%-30rem)]"
        >
          <div
            style={{
              clipPath:
                "polygon(74.1% 44.1%, 100% 61.6%, 97.5% 26.9%, 85.5% 0.1%, 80.7% 2%, 72.5% 32.5%, 60.2% 62.4%, 52.4% 68.1%, 47.5% 58.3%, 45.2% 34.5%, 27.5% 76.7%, 0.1% 64.9%, 17.9% 100%, 27.6% 76.8%, 76.1% 97.7%, 74.1% 44.1%)",
            }}
            className="relative left-[calc(50%+3rem)] aspect-[1155/678] w-[36.125rem] -translate-x-1/2 bg-gradient-to-tr from-[#ff80b5] to-[#9089fc] opacity-30 sm:left-[calc(50%+36rem)] sm:w-[72.1875rem]"
          />
        </div>
      </div>
      <div className="overflow-hidden bg-white  sm:py-32">
        <div className="mx-auto max-w-7xl px-6 lg:px-8">
          <div className="mx-auto grid max-w-2xl grid-cols-1 gap-x-8 gap-y-16 sm:gap-y-20 lg:mx-0 lg:max-w-none lg:grid-cols-2">
            <div className="lg:ml-auto lg:pl-4 lg:pt-4">
              <div className="lg:max-w-lg">
                <h2 className="text-base font-semibold leading-7 text-indigo-600">
                  Fast, Safe, Legal
                </h2>
                <p className="mt-2 text-pretty text-4xl font-semibold tracking-tight text-gray-900 sm:text-5xl">
                  A better workflow
                </p>
                <p className="mt-6 text-lg leading-8 text-gray-600">
                  TOKIFY Protocol is a smart contract-based platform on the
                  MANTRA Chain that enables the tokenization of real-world
                  assets (RWAs) through smart contracts and decentralized
                  governance. Asset owners can tokenize physical assets like
                  real estate, allowing fractional ownership and providing
                  investors with liquid, accessible stakes in traditionally
                  illiquid markets.
                </p>
                <dl className="mt-10 max-w-xl space-y-8 text-base leading-7 text-gray-600 lg:max-w-none">
                  {features.map((feature) => (
                    <div key={feature.name} className="relative pl-9">
                      <dt className="inline font-semibold text-gray-900">
                        <feature.icon
                          aria-hidden="true"
                          className="absolute left-1 top-1 h-5 w-5 text-indigo-600"
                        />
                        {feature.name}
                      </dt>{" "}
                      <dd className="inline">{feature.description}</dd>
                    </div>
                  ))}
                </dl>
              </div>
            </div>
            <div className="flex items-start justify-end lg:order-first">
              <img
                alt="Product screenshot"
                src="01_legal_framework.png"
                className=" rounded-xl shadow-xl ring-1 ring-gray-400/10 sm:w-[57rem]"
              />
            </div>
          </div>
        </div>
      </div>
      <div>
        <p className="text-center pb-10 mt-2 text-pretty text-4xl font-semibold tracking-tight text-gray-900 sm:text-5xl">
          Trusts
        </p>
        <div className="flex justify-center mb-10">
          <ul role="list" className="grid grid-cols-1 md:grid-cols-3 gap-x-4">
            {trusts.map((file) => (
              <Link href={`trust/${file._id}`}>
                <TokenizeTrustCard
                  imageUrl={file.metadata.image}
                  name={file.metadata.name}
                  category={file.metadata.category}
                  numberOfProperties={25}
                  portfolioValue={50000000}
                  numberOfInvestors={1500}
                  apy={7.2}
                />
              </Link>
            ))}
          </ul>
        </div>
      </div>
      <div>
        <p className="text-center pb-10 mt-2 text-pretty text-4xl font-semibold tracking-tight text-gray-900 sm:text-5xl">
          Assets
        </p>
        <div className="flex justify-center">
          <ul
            role="list"
            className="grid  grid-cols-1 md:grid-cols-3 gap gap-x-4 "
          >
            {assets.map((file) => (
              <Link href={`asset/${file._id}`}>
                <TokenizedAssetCard
                  imageUrl={file.image_uri}
                  title={file.name}
                  pricePerShare={file.price_per_share.amount}
                  apy={file.estimated_apy}
                  monthlyIncome={file.estimated_monthly_income.amount}
                  category={file.subcategory}
                />
              </Link>
            ))}
          </ul>
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
