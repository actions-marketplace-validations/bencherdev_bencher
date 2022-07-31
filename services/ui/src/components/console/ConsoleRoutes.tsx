import {
  createSignal,
  createEffect,
  lazy,
  Component,
  createMemo,
  Accessor,
  Signal,
  For,
  createResource,
} from "solid-js";
import validator from "validator";
import { Routes, Route, Navigate, useLocation } from "solid-app-router";
import { JsonUser } from "bencher_json";
import { Operation, Resource, Button, Field } from "./console";
import AccountPage from "../site/account/AccountPage";
import validateName from "../fields/validators/validateName";
import validateDescription from "../fields/validators/validateDescription";

const ConsolePage = lazy(() => import("./ConsolePage"));

const getConfig = (pathname) => {
  console.log(pathname);
  return {
    [Resource.PROJECTS]: {
      [Operation.LIST]: {
        operation: Operation.LIST,
        title: "Projects",
        header: "name",
        items: [
          {
            kind: "text",
            key: "slug",
          },
          {},
          {
            kind: "bool",
            key: "owner_default",
            text: "Default",
          },
          {},
        ],
        buttons: [
          { kind: Button.ADD, path: "/console/projects/add" },
          { kind: Button.REFRESH },
        ],
      },
      [Operation.ADD]: {
        operation: Operation.ADD,
        title: "Add Project",
        fields: [
          {
            kind: Field.INPUT,
            key: "name",
            label: true,
            value: "",
            valid: null,
            validate: true,
            clear: false,
            config: {
              label: "Name",
              type: "text",
              placeholder: "Project Name",
              icon: "fas fa-project-diagram",
              help: "Must be at least four characters or longer.",
              validate: validateName,
            },
          },
          {
            kind: Field.TEXTAREA,
            key: "description",
            label: true,
            value: "",
            valid: null,
            validate: true,
            clear: false,
            config: {
              label: "Description",
              type: "textarea",
              placeholder: "Describe the project",
              help: "Must be between 25 and 2,500 characters.",
              validate: validateDescription,
            },
          },
          {
            kind: Field.INPUT,
            key: "url",
            label: true,
            value: "",
            valid: null,
            validate: true,
            clear: false,
            config: {
              label: "URL",
              type: "text",
              placeholder: "www.example.com",
              icon: "far fa-window-maximize",
              help: "Must be a valid public facing URL.",
              validate: validator.isURL,
            },
          },
        ],
        buttons: {
          [Button.BACK]: { path: "/console/projects" },
        },
      },
    },
  };
};

const ConsoleRoutes = (props) => {
  const [config] = createResource(props.pathname, getConfig);

  return (
    <>
      {/* Console Routes */}
      <Route path="/" element={<Navigate href={"/console/projects"} />} />
      {/* Console Projects Routes */}
      <Route
        path="/projects"
        element={
          <ConsolePage
            config={config()?.[Resource.PROJECTS]?.[Operation.LIST]}
            pathname={props.pathname}
            handleTitle={props.handleTitle}
            handleRedirect={props.handleRedirect}
          />
        }
      />
      <Route
        path="/projects/add"
        element={
          <ConsolePage
            config={config()?.[Resource.PROJECTS]?.[Operation.ADD]}
            pathname={props.pathname}
            handleTitle={props.handleTitle}
            handleRedirect={props.handleRedirect}
          />
        }
      />
      <Route
        path="/projects/:project_slug"
        element={
          <ConsolePage
            config={config()?.[Resource.PROJECTS]?.[Operation.VIEW]}
            pathname={props.pathname}
            handleTitle={props.handleTitle}
            handleRedirect={props.handleRedirect}
          />
        }
      />
      <Route
        path="/projects/:project_slug/perf"
        element={
          <ConsolePage
            config={config()?.[Resource.PROJECTS]?.[Operation.PERF]}
            pathname={props.pathname}
            handleTitle={props.handleTitle}
            handleRedirect={props.handleRedirect}
          />
        }
      />
      <Route path="/account" element={<AccountPage />} />
    </>
  );
};

export default ConsoleRoutes;