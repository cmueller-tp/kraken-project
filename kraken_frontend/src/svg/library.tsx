import React from "react";

type LibraryIconProps = {};

export default function LibraryIcon(props: LibraryIconProps) {
    return (
        <div className={"icon"} {...props}>
            <svg
                className="neon"
                fill="none"
                stroke="#000"
                strokeLinejoin="round"
                width="800px"
                height="800px"
                strokeWidth="32px"
                viewBox="0 0 512 512"
                xmlns="http://www.w3.org/2000/svg"
            >
                <rect x="32" y="96" width="64" height="368" rx="16" ry="16" />
                <line x1="112" y1="224" x2="240" y2="224" strokeLinecap="round" />
                <line x1="112" y1="400" x2="240" y2="400" strokeLinecap="round" />
                <rect x="112" y="160" width="128" height="304" rx="16" ry="16" />
                <rect x="256" y="48" width="96" height="416" rx="16" ry="16" />
                <path d="M422.46,96.11l-40.4,4.25c-11.12,1.17-19.18,11.57-17.93,23.1l34.92,321.59c1.26,11.53,11.37,20,22.49,18.84l40.4-4.25c11.12-1.17,19.18-11.57,17.93-23.1L445,115C443.69,103.42,433.58,94.94,422.46,96.11Z" />
            </svg>
        </div>
    );
}
